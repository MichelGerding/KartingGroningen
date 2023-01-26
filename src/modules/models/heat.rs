use std::collections::{HashMap, HashSet};
use std::thread;

use crate::errors::HeatInvalidError;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::{Double, Integer, Timestamp, VarChar};
use diesel::{sql_query, Identifiable, NotFound, PgConnection, Queryable};
use identifiable_derive::HasId;
use redis::Commands;
use serde::{Deserialize, Serialize};

use crate::modules::models::driver::Driver;
use crate::modules::models::kart::Kart;
use crate::modules::models::lap::{Lap, LapsStats};
use crate::modules::redis::Redis;
use crate::modules::traits::as_map::AsMap;
use crate::modules::traits::has_id::HasIdTrait;
use crate::schema::{heats, laps};

use rocket::response;
use rocket::Request;
use rocket::response::Responder;
use rocket::response::Response;
use rocket::http::ContentType;
use json_response_derive::JsonResponse;
use skillratings::MultiTeamOutcome;
use skillratings::weng_lin::{weng_lin_multi_team, WengLinConfig, WengLinRating};


#[derive(Insertable, Serialize, Debug, Clone, Deserialize)]
#[diesel(table_name = heats)]
pub struct NewHeat {
    pub heat_id: String,
    pub heat_type: String,
    pub start_date: NaiveDateTime,
}

#[derive(
    Queryable, Serialize, Identifiable, PartialEq, Debug, Clone, Deserialize, Eq, Hash, HasId,
)]
pub struct Heat {
    pub id: i32,
    pub heat_id: String,
    pub heat_type: String,
    pub start_date: NaiveDateTime,
}

impl Heat {
    /// # create heat
    /// create a new heat. this function panics if the heat already exists
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `heat_id` - the heat id
    /// * `heat_type` - the heat type
    /// * `start_date` - the start date of the heat
    ///
    /// ## Returns
    /// * `Heat` - the created heat
    ///
    pub fn new(
        conn: &mut PgConnection,
        heat_id_in: &str,
        heat_type_in: &str,
        start_date_in: &str,
    ) -> Heat {
        use crate::schema::heats::dsl::*;

        let timestamp =
            NaiveDateTime::parse_from_str(start_date_in, "%Y-%m-%dT%H:%M:%S%.f%z").unwrap();
        let new_heat = NewHeat {
            heat_id: heat_id_in.to_string(),
            heat_type: heat_type_in.to_string(),
            start_date: timestamp,
        };

        let heat: Heat = diesel::insert_into(heats)
            .values(&new_heat)
            .get_result(conn)
            .expect("Error saving new heat");


        heat.clear_cache(&mut Redis::connect());

        heat
    }

    /// # check if exists
    /// check if a heat exists
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `heat_id` - the heat id
    ///
    /// ## Returns
    /// * `bool` - true if the heat exists
    pub fn exists(conn: &mut PgConnection, heat_id_in: &str) -> bool {
        use crate::schema::heats::dsl::*;
        use diesel::dsl::exists;
        use diesel::select;

        select(exists(heats.filter(heat_id.like(heat_id_in))))
            .get_result(conn)
            .unwrap()
    }

    /// # delete heat
    /// delete a heat
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    pub fn delete(&self, conn: &mut PgConnection) {
        Heat::delete_db_id(conn, self.id);
    }

    /// # delete heat by id
    /// delete the heat with the given id
    /// the given id is the heat_id not the database id
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `heat_id` - the id of the heat to delete
    pub fn delete_id(conn: &mut PgConnection, heat_id: &str) {
        match Heat::get_by_id(conn, heat_id) {
            Ok(heat) => heat.delete(conn),
            Err(_) => {}
        };
    }

    /// # delete heat by db id
    /// delete the heat with the given database id
    /// the given id is the database id not the heat_id
    ///
    /// this function also deletes all the laps associated with the heat
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `db_id` - the database id of the heat to delete
    pub fn delete_db_id(conn: &mut PgConnection, db_id: i32) {

        let heat = Heat::get_by_db_id(conn, db_id);
        let laps = Lap::from_heat(conn, &heat);
        let drivers = Driver::from_laps(conn, &laps);

        thread::spawn(move || {
            let r_conn = &mut Redis::connect();
            for driver in drivers {
                driver.clear_cache(r_conn);
            }

            heat.clear_cache(r_conn);
        });

        // delete all laps in one query
        diesel::delete(laps::table.filter(laps::heat.eq(db_id)))
            .execute(conn)
            .expect("Error deleting laps");

        // delete the heat
        diesel::delete(heats::table.filter(heats::id.eq(db_id)))
            .execute(conn)
            .expect("Error deleting heat");
    }

    pub fn clear_cache(&self, r_conn: &mut redis::Connection) {
        // get all keys
        let mut keys = r_conn
            .keys::<String, Vec<String>>(format!("*{}*", self.heat_id))
            .expect("Error getting keys from redis");

        keys.append(&mut vec![
            "/api/drivers/all".to_string(),
            "/api/drivers/all/full".to_string(),
            "/api/heats/all".to_string(),
            "/api/heats/all/full".to_string(),
            "/api/heats/all".to_string(),
            "/api/heats/all/full".to_string(),
            "/heats/all".to_string(),
        ]);

        // delete all keys
        for key in keys {
            r_conn.del::<String, ()>(key).expect("Error deleting key");
        }
    }

    /// # get heat by id
    /// get the heat with the given id
    /// the given id is the database id not the heat_id
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `db_id` - the database id of the heat to get
    ///
    /// ## Returns
    /// * `Heat` - the heat
    pub fn get_by_db_id(conn: &mut PgConnection, db_id: i32) -> Heat {
        Heat::get_from_db_ids(conn, &[db_id]).pop().unwrap()
    }

    /// # get from db ids
    /// get the heats with the given database ids
    /// the given ids are the database ids not the heat_ids
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `db_ids` - the database ids of the heats to get
    ///
    /// ## Returns
    /// * `Vec<Heat>` - the heats with the given database ids
    pub fn get_from_db_ids(conn: &mut PgConnection, db_ids: &[i32]) -> Vec<Heat> {
        use crate::schema::heats::dsl::*;

        heats
            .filter(id.eq_any(db_ids))
            .load::<Heat>(conn)
            .expect("Error loading heat")
    }

    /// # get the heats from a list of laps
    /// get the heats from a list of laps
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `laps` - the laps
    ///
    /// ## Returns
    /// * `Vec<Heat>` - the heats
    pub fn from_laps(conn: &mut PgConnection, laps: &[Lap]) -> Vec<Heat> {
        let heat_ids = laps.iter().map(|e| e.heat).collect::<Vec<i32>>();

        Heat::get_from_db_ids(conn, &heat_ids)
    }

    /// # get the heats from a list of laps
    /// get the heats from a list of laps
    ///
    /// ## Arguments
    /// * `laps` - the laps
    ///
    /// ## Returns
    /// * `Vec<Heat>` - the heats
    pub fn from_laps_offline(heats: &[Heat], laps: &[Lap]) -> Vec<Heat> {
        let heats_map: HashMap<i32, Heat> = heats.iter().map(|e| (e.id, e.to_owned())).collect();

        let mut ret = HashSet::new();
        for lap in laps {
            ret.insert(heats_map.get(&lap.heat).unwrap());
        }

        ret.iter().map(|e| e.to_owned().to_owned()).collect()
    }

    /// # get by id
    /// get the heat with the given id
    /// the given id is the heat_id not the database id
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `heat_id_in` - the id of the heat to get
    ///
    /// ## Returns
    /// * `Heat` - the heat
    pub fn get_by_id(conn: &mut PgConnection, heat_id_in: &str) -> Result<Heat, HeatInvalidError> {
        use crate::schema::heats::dsl::*;

        match heats.filter(heat_id.like(heat_id_in)).first::<Heat>(conn) {
            Ok(heat) => Ok(heat),
            Err(NotFound) => Err(HeatInvalidError::new(format!(
                "Heat: {} not found",
                heat_id_in
            ))),
            Err(_) => Err(HeatInvalidError::new("Unknown error".to_string())),
        }
    }

    /// # get all heats
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    ///
    /// ## Returns
    /// * `Vec<Heat>` - all the heats
    pub fn get_all(conn: &mut PgConnection) -> Vec<Heat> {
        use crate::schema::heats::dsl::*;

        heats.load::<Heat>(conn).expect("Error loading heats")
    }

    /// # get the laps of the heat
    /// get the laps of the heat
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    ///
    /// ## Returns
    /// * `Vec<Lap>` - the laps of the heat
    pub fn get_laps(&self, conn: &mut PgConnection) -> Vec<Lap> {
        Lap::from_heat(conn, self)
    }

    /// # get all heats with stats
    /// get all heats with basic stats.
    /// the stats given are the number of laps and the number of drivers
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    ///
    /// ## Returns
    /// * `Vec<HeatStats>` - all the heats with stats
    pub fn get_all_with_stats(conn: &mut PgConnection) -> Vec<HeatStats> {
        sql_query(
            "
        select
            h.id,
            h.heat_id,
            h.heat_type,
            h.start_date as start_time,
            CAST(count(l.*) as INT) as amount_of_laps,
            CAST(count(DISTINCT l.driver) AS INT) as amount_of_drivers,
            min(l.lap_time) as fastest_lap_time,
            avg(l.lap_time) as average_lap_time
        from heats h
        inner join laps l on h.id = l.heat
        group by h.id
        ",
        )
        .load::<HeatStats>(conn)
        .expect("Error loading heats")
    }

    /// # get a single heat with stats
    /// get a single heat with the basic stats: lap count, driver count,
    /// fastest lap, and average lap
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `heat_id` - the id of heat to search
    ///
    /// ## Returns
    /// * `HeatStats` - heat and its stats
    pub fn get_with_stats(conn: &mut PgConnection, heat_id: String) -> HeatStats {
        sql_query(
            "
        select
            h.id,
            h.heat_id,
            h.heat_type,
            h.start_date as start_time,
            CAST(count(l.*) as INT) as amount_of_laps,
            CAST(count(DISTINCT l.driver) AS INT) as amount_of_drivers,
            min(l.lap_time) as fastest_lap_time,
            avg(l.lap_time) as average_lap_time
        from heats h
                 inner join laps l on h.id = l.heat
        where h.heat_id = 'BB4E21447521429EBBCD7602BB08BEF0'
        group by h.id
        ",
        )
        .bind::<VarChar, _>(heat_id)
        .get_result(conn)
        .expect("TODO: panic message")
    }

    /// # get all heats sorted by date
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    ///
    /// ## Returns
    /// * `Vec<Heat>` - all the heats sorted by date
    pub fn get_all_chronologicaly(conn: &mut PgConnection) -> Vec<Heat> {
        use crate::schema::heats::dsl::*;

        heats
            .order(start_date.asc())
            .load::<Heat>(conn)
            .expect("Error loading heats")
    }

    /// # ensure a heat exists
    /// ensure a heat exists
    /// if the heat does not exist it will be created
    /// this function is prefered over `new` because it wont panic
    /// if the heat already exists
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `heat_id` - the id of the heat
    /// * `heat_type` - the type of the heat
    /// * `start_date` - the start date of the heat
    ///
    /// ## Returns
    /// * `Heat` - the heat
    pub fn ensure_exists(
        conn: &mut PgConnection,
        heat_id: &str,
        heat_type: &str,
        start_time: &str,
    ) -> Heat {
        if !Heat::exists(conn, heat_id) {
            Heat::new(conn, heat_id, heat_type, start_time)
        } else {
            match Heat::get_by_id(conn, heat_id) {
                Ok(heat) => heat,
                Err(e) => panic!("{}", e.to_string()),
            }
        }
    }

    /// # convert to new heat
    /// convert the current heat to a NewHeat object
    ///
    /// ## Returns
    /// * `NewHeat` - the new heat
    pub fn to_new(&self) -> NewHeat {
        NewHeat {
            heat_id: self.heat_id.clone(),
            heat_type: self.heat_type.clone(),
            start_date: self.start_date,
        }
    }

    /// # get laps per driver
    /// get all laps driven by each driver in the heat
    /// the function returns a hashmap that uses the driver as key and laps as value
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    ///
    /// ## Returns
    /// * `HashMap<String, Vec<Lap>>` - the laps per driver
    pub fn laps_per_driver(&self, conn: &mut PgConnection) -> HashMap<Driver, Vec<Lap>> {
        let v_laps = Lap::from_heat(conn, self);
        let v_drivers = Driver::from_laps(conn, &v_laps);

        Heat::parse_laps_and_drivers_into_map(&v_laps, &v_drivers)
    }

    /// # get driver stats
    /// get the stats of all drivers in the heat
    /// the function returns a hashmap that uses the driver as key and the stats as value
    pub fn get_driver_stats(&self, conn: &mut PgConnection) -> HashMap<Driver, LapsStats> {
        let laps_per_driver = self.laps_per_driver(conn);

        let mut driver_stats = HashMap::new();
        for (driver, laps) in laps_per_driver {
            driver_stats.insert(driver.to_owned(), Lap::get_stats_of_laps(&laps.to_owned()));
        }

        driver_stats
    }

    /// # get amount of heats
    /// get the amount of heats that a vec of laps are driven in.
    ///
    /// ## Arguments
    /// * `laps` - the laps to get the amount of heats for
    ///
    /// ## Returns
    /// * `i32` - the amount of heats
    pub fn amount_from_laps(laps: &Vec<Lap>) -> i32 {
        // get the unique amount of heats from the laps
        let mut heat_ids = HashSet::new();
        for lap in laps {
            heat_ids.insert(lap.heat);
        }

        heat_ids.len() as i32
    }

    /// # filter laps
    /// filter the laps with a different heat from the current heat
    /// this function is used to filter out laps that are not in the current heat
    ///
    /// ## Arguments
    /// * `laps` - the laps to filter
    ///
    /// ## Returns
    /// * `Vec<Lap>` - the filtered laps
    pub fn filter_other_heat_laps(&self, laps: &Vec<Lap>) -> Vec<Lap> {
        let mut heat_laps = Vec::new();
        for lap in laps {
            if lap.heat == self.id {
                heat_laps.push(lap.to_owned());
            }
        }

        heat_laps
    }

    pub fn get_full_info(&self, connection: &mut PgConnection) -> FullHeatInfo {
        let laps = Lap::from_heat(connection, self);
        let drivers: Vec<Driver> = Driver::from_laps(connection, &laps);
        let karts: Vec<Kart> = Kart::from_laps(connection, &laps);

        let laps_per_driver = Heat::parse_laps_and_drivers_into_map(&laps, &drivers);

        let mut full_heat_info = FullHeatInfo {
            id: self.id,
            heat_id: self.heat_id.to_owned(),
            heat_type: self.heat_type.to_owned(),
            start_time: self.start_date,
            drivers: Vec::new(),
        };

        for (driver, laps) in laps_per_driver {
            full_heat_info.drivers.push(HeatDriverInfo {
                id: driver.id,
                name: driver.name,
                kart: karts
                    .iter()
                    .find(|kart| kart.id == laps[0].kart_id)
                    .unwrap()
                    .to_owned(),
                laps: laps.to_owned(),
            });
        }

        full_heat_info
    }

    /// # parse laps and drivers into map
    /// parse the laps and drivers into a hashmap
    ///
    /// ## Arguments
    /// * `laps` - the laps to parse
    /// * `drivers` - the drivers to parse
    ///
    /// ## Returns
    /// * `HashMap<Driver, Vec<Lap>>` - the parsed hashmap
    fn parse_laps_and_drivers_into_map(
        laps: &Vec<Lap>,
        drivers: &Vec<Driver>,
    ) -> HashMap<Driver, Vec<Lap>> {
        let mut laps_per_driver = HashMap::new();
        let driver_map = drivers.to_owned().into_iter().as_map();
        for lap_reference in laps {
            let lap = lap_reference.to_owned();
            let driver: Driver = driver_map.get(&lap.driver).unwrap().to_owned();

            if let std::collections::hash_map::Entry::Vacant(e) =
                laps_per_driver.entry(driver.to_owned())
            {
                e.insert(vec![lap]);
            } else {
                laps_per_driver.get_mut(&driver).unwrap().push(lap);
            }
        }

        laps_per_driver
    }

    /// # get laps per heat
    /// get all laps per heat in the given vec as a hashmap
    ///
    /// ## Arguments
    /// * `heats` - the heats to get the laps per heat from
    /// * `laps` - the laps to get the laps per heat from
    ///
    /// ## Returns
    /// * `HashMap<Heat, Vec<Lap>>` - the laps per heat
    pub fn get_laps_per_heat(heats: &[Heat], laps: &[Lap]) -> HashMap<Heat, Vec<Lap>> {
        let heat_map = heats.to_owned().into_iter().as_map();

        let mut heat_laps_map = HashMap::new();
        for lap_reference in laps {
            let lap = lap_reference.to_owned();
            let heat: Heat = heat_map.get(&lap.heat).unwrap().to_owned();

            if let std::collections::hash_map::Entry::Vacant(e) =
                heat_laps_map.entry(heat.to_owned())
            {
                e.insert(vec![lap]);
            } else {
                heat_laps_map.get_mut(&heat).unwrap().push(lap);
            }
        }

        heat_laps_map
    }


    pub fn apply_ratings(&self, connection: &mut PgConnection) {
        #[derive(QueryableByName, Debug)]
        struct LocalRating {
            #[diesel(sql_type = Integer)]
            id: i32,
            #[diesel(sql_type = Double)]
            rating: f64,
            #[diesel(sql_type = Double)]
            uncertainty: f64,
        }

        // get the order the drivers finished in the heat
        let drivers: Vec<LocalRating> = sql_query(format!("
            SELECT d.id, d.rating, d.uncertainty from drivers d
                inner join laps l on d.id = l.driver
                inner join heats h on l.heat = h.id
            where heat_id = '{}'
            group by d.id
            order by min(l.lap_time) asc
        ", self.heat_id))
            .load::<LocalRating>(connection)
            .unwrap();

        println!("drivers: {:?}", drivers);

        let teams: Vec<Vec<WengLinRating>> = drivers
            .iter()
            .map(|driver| {
                return vec![WengLinRating {
                    rating: driver.rating,
                    uncertainty: driver.uncertainty
                }]
            }).collect();

        // create the ratingh groups
        let mut rating_groups = Vec::new();
        for (position, _) in drivers.iter().enumerate() {
            let result = MultiTeamOutcome::new(position + 1);

            rating_groups.push((
                &(teams[position])[..],
                result
                ));
        }

        let new_ratings = weng_lin_multi_team(&rating_groups[..], &WengLinConfig::default());
        for (position, driver) in drivers.iter().enumerate() {
            let new_rating = &new_ratings[position];
            println!("{}: {} -> {}", driver.id, driver.rating, new_rating[0].rating);
            Driver::set_rating_id(connection, driver.id, new_rating[0]);

        }
    }
}

#[derive(QueryableByName, Serialize, Deserialize, JsonResponse)]
pub struct HeatStats {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = VarChar)]
    pub heat_id: String,
    #[diesel(sql_type = VarChar)]
    pub heat_type: String,
    #[diesel(sql_type = Timestamp)]
    pub start_time: NaiveDateTime,
    #[diesel(sql_type = Integer)]
    pub amount_of_laps: i32,
    #[diesel(sql_type = Integer)]
    pub amount_of_drivers: i32,
    #[diesel(sql_type = Double)]
    pub fastest_lap_time: f64,
    #[diesel(sql_type = Double)]
    pub average_lap_time: f64,
}

#[derive(Debug)]
pub struct FullHeatInfo {
    pub id: i32,
    pub heat_id: String,
    pub heat_type: String,
    pub start_time: NaiveDateTime,
    pub drivers: Vec<HeatDriverInfo>,
}

#[derive(Debug)]
pub struct HeatDriverInfo {
    pub id: i32,
    pub name: String,
    pub kart: Kart,
    pub laps: Vec<Lap>,
}
