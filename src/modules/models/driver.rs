use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use diesel::dsl::{exists};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_types::{Double, Integer, VarChar};
use diesel::{NotFound, select, sql_query};
use diesel::result::Error;
use serde::{Deserialize, Serialize};

use crate::modules::helpers::general::Helpers;
use crate::modules::helpers::math::Math;
use crate::modules::models::heat::Heat;
use crate::modules::models::kart::Kart;
use crate::modules::models::lap::Lap;
use crate::modules::traits::as_map::AsMap;
use crate::modules::traits::has_id::HasIdTrait;
use crate::schema::*;
use crate::{TemplateDataDriver, TemplateDataLap};
use crate::modules::redis::Redis;

use identifiable_derive::HasId;
use redis::Commands;
use regex::Regex;

use rocket::response;
use rocket::Request;
use rocket::response::Responder;
use rocket::response::Response;
use rocket::http::ContentType;
use json_response_derive::JsonResponse;
use skillratings::weng_lin::WengLinRating;


trait IdentifiableAsMap {
    fn get_id(&self) -> i32;
}

#[derive(Insertable, Serialize, Debug, Clone, Deserialize)]
#[diesel(table_name = drivers)]
pub struct NewDriver {
    pub name: String,
    pub rating: f64,
    pub uncertainty: f64,
}

#[derive(Queryable, Serialize, Identifiable, Debug, Clone, Deserialize, HasId)]
pub struct Driver {
    pub id: i32,
    pub name: String,
    pub rating: f64,
    pub uncertainty: f64,
}

impl Hash for Driver {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Driver {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Driver {}

impl Driver {
    /// # Create a new driver
    /// create a new entry in the databse for a driver.
    /// this function does not check if the driver already exists.
    /// if you want to make sure the driver exists, use the `ensure_exists` function.
    ///
    /// ## Arguments
    /// * `connection` - the database connection
    /// * `name` - the name of the driver
    ///
    /// ## Returns
    /// * `Result<Driver, diesel::result::Error>` - the driver that was created
    pub fn new(connection: &mut PgConnection, name: &str) -> Driver {
        let sanitized_name = sanitize_name(name);
        let new_driver = NewDriver {
            name: sanitized_name.to_string(),
            rating: 25.0,
            uncertainty: 25.0/3.0,
        };

        let driver: Driver = diesel::insert_into(drivers::table)
            .values(&new_driver)
            .get_result(connection)
            .unwrap();

        driver.clear_cache(&mut Redis::connect());


        driver
    }

    /// # Get all drivers
    /// get all drivers from the database
    ///
    /// ## Arguments
    /// * `connection` - the database connection
    ///
    /// ## Returns
    /// * `Vec<Driver> - all drivers
    pub fn get_all(connection: &mut PgConnection) -> Vec<Driver> {
        use crate::schema::drivers::dsl::*;

        drivers
            .load::<Driver>(connection)
            .expect("Error loading drivers")
    }

    /// # Get a driver by id
    /// get a driver by id from the database
    /// if the driver does not exist, this function will panic
    ///
    /// ## Arguments
    /// * `connection` - the database connection
    /// * `id_in` - the id of the driver
    ///
    /// ## Returns
    /// * `Driver` - the driver
    pub fn get_by_id(connection: &mut PgConnection, id_in: i32) -> Driver {
        use crate::schema::drivers::dsl::*;

        drivers
            .find(id_in)
            .first::<Driver>(connection)
            .expect("Error loading driver")
    }

    pub fn get_by_ids(connection: &mut PgConnection, ids: Vec<i32>) -> Vec<Driver> {
        use crate::schema::drivers::dsl::*;

        drivers
            .filter(id.eq_any(ids))
            .load::<Driver>(connection)
            .unwrap()
    }

    pub fn search_with_stats_paginated(conn: &mut PgConnection, driver_name: String, page_size: u32, page: u32) -> Result<Vec<DriverStats>, Error> {
        match sql_query(format!("
            select
                d.name,
                d.rating,
                min(l.lap_time) as fastest_lap_time,
                avg(l.lap_time) as avg_lap_time,
                percentile_cont(0.5) WITHIN GROUP ( ORDER BY l.lap_time) as median_lap_time,
                CAST(count(l.lap_time) AS INT) as total_laps,
                CAST(count(DISTINCT l.heat) AS INT) as total_heats
            from drivers d
                     inner join laps l on d.id = l.driver
            where d.name like '%{}%'
            GROUP BY d.name, d.rating
            limit {} offset {}",
            driver_name,
            page_size,
            page * page_size,

        ))
            .load::<DriverStats>(conn)
        {
            Ok(d) => Ok(d),
            Err(_) => Err(NotFound),
        }
    }

    pub fn search_with_stats(conn: &mut PgConnection, driver_name: String) -> Result<Vec<DriverStats>, Error> {
        match sql_query(format!("
            select
                d.name,
                d.rating,
                min(l.lap_time) as fastest_lap_time,
                avg(l.lap_time) as avg_lap_time,
                percentile_cont(0.5) WITHIN GROUP ( ORDER BY l.lap_time) as median_lap_time,
                CAST(count(l.lap_time) AS INT) as total_laps,
                CAST(count(DISTINCT l.heat) AS INT) as total_heats
            from drivers d
            inner join laps l on d.id = l.driver
            where d.name like '%{}%'
            GROUP BY d.name, d.rating;",
            driver_name
        ))
        .load::<DriverStats>(conn)
        {
            Ok(d) => Ok(d),
            Err(_) => Err(NotFound),
        }
    }

    /// # Get all drivers with stats
    /// get all drivers from the database with stats
    /// this function gets some statistics over all drivers.
    /// it gives fastest, average, and median lap times. total amount of laps and heats.
    /// it also gives you there name
    ///
    /// this is a very expensive function, and should not be used in a loop.
    ///
    /// ## Arguments
    /// * `connection` - the database connection
    ///
    /// ## Returns
    /// * `Vec<DriverStats> - stats of all drivers
    pub fn get_all_with_stats(conn: &mut PgConnection) -> Vec<DriverStats> {
        sql_query(
            "
            select
                d.name,
                d.rating,
                min(l.lap_time) as fastest_lap_time,
                avg(l.lap_time) as avg_lap_time,
                percentile_cont(0.5) WITHIN GROUP ( ORDER BY l.lap_time) as median_lap_time,
                CAST(count(l.lap_time) AS INT) as total_laps,
                CAST(count(DISTINCT l.heat) AS INT) as total_heats
            from drivers d
            inner join laps l on d.id = l.driver
            GROUP BY d.name, d.rating;",
        )
            .load::<DriverStats>(conn)
        .expect("Error loading driver stats")
    }

    /// # Get driver with stats
    /// get driver from the database with stats
    /// this function gets some statistics of a driver.
    /// it gives fastest, average, and median lap times. total amount of laps and heats.
    /// it also gives you there name
    ///
    /// ## Arguments
    /// * `connection` - the database connection
    /// * `driver_name` - the name of the driver
    ///
    /// ## Returns
    /// * `Vec<DriverStats> - stats of all drivers
    pub fn get_driver_with_stats(conn: &mut PgConnection, driver_name: String) -> DriverStats {
        sql_query(format!(
            "
            select
                d.name,
                d.rating,
                min(l.lap_time) as fastest_lap_time,
                avg(l.lap_time) as avg_lap_time,
                percentile_cont(0.5) WITHIN GROUP ( ORDER BY l.lap_time) as median_lap_time,
                CAST(count(l.lap_time) AS INT) as total_laps,
                CAST(count(DISTINCT l.heat) AS INT) as total_heats
            from drivers d
            inner join laps l on d.id = l.driver
            where d.name = '{}'
            GROUP BY d.name, d.rating;",
            driver_name
        ))
        .get_result::<DriverStats>(conn)
        .expect("Error getting driver stats")
    }

    /// # check if a driver exists
    /// check if a driver exists in the database
    ///
    /// ## Arguments
    /// * `connection` - the database connection
    /// * `name_in` - the name of the driver
    ///
    /// ## Returns
    /// * `bool` - true if the driver exists, false if not
    pub fn exists(conn: &mut PgConnection, name_in: &String) -> bool {
        use crate::schema::drivers::dsl::*;
        select(exists(drivers.filter(name.like(name_in))))
            .get_result(conn)
            .unwrap()
    }

    /// # get a driver by name
    /// get a driver by name. if the driver does not exists it will panic
    ///
    /// ## Arguments
    /// * `connection` - the database connection
    ///
    /// ## Returns
    /// * `Driver` - the driver
    pub fn get_by_name(conn: &mut PgConnection, name_in: &String) -> Result<Driver, Error> {
        use crate::schema::drivers::dsl::*;
        drivers
            .filter(name.like(name_in))
            .first::<Driver>(conn)
    }

    pub fn search_by_name(conn: &mut PgConnection, name_in: &String) -> Result<Vec<Driver>,Error> {
        use crate::schema::drivers::dsl::*;

        drivers
            .filter(name.like(format!("%{}%", name_in)))
            .load::<Driver>(conn)
    }

    pub fn search_by_name_paginated(conn: &mut PgConnection, name_in: &String, page: i32, page_size:i32) -> Result<Vec<Driver>,Error> {
        use crate::schema::drivers::dsl::*;

        drivers
            .filter(name.like(format!("%{}%", name_in)))
            .limit(page_size as i64)
            .offset((page * page_size) as i64)
            .load::<Driver>(conn)
    }

    /// # get the stats of a driver
    /// get the stats of a driver. this function is the same as get_all_with_stats, but only for one driver
    /// this function can only be called on a driver object. if the driver does not exists it will panic
    ///
    /// ## Arguments
    /// * `connection` - the database connection
    ///
    /// ## Returns
    /// * `DriverStats` - the stats of the driver
    pub fn get_stats(&self, conn: &mut PgConnection) -> DriverStats {
        sql_query(
            "
            select
                d.name,
                d.rating,
                min(l.lap_time) as fastest_lap_time,
                avg(l.lap_time) as avg_lap_time,
                percentile_cont(0.5) WITHIN GROUP ( ORDER BY l.lap_time) as median_lap_time,
                CAST(count(l.lap_time) AS INT) as total_laps,
                CAST(count(DISTINCT l.heat) AS INT) as total_heats
            from drivers d
            inner join laps l on d.id = l.driver
            where d.id = $1
            GROUP BY d.name, d.rating;",
        )
        .bind::<Integer, _>(self.id)
        .get_result::<DriverStats>(conn)
        .expect("Error loading driver stats")
    }

    /// # ensure a driver exists
    /// ensure a driver exists in the database. if the driver does not exists it will be created
    /// this function is preferred to `new`. this function will not panic if the driver already exists.
    /// if the driver exists they will be returned instead of created.
    ///
    /// if performance is a concern, use `new` instead.
    ///
    /// ## Arguments
    /// * `connection` - the database connection
    /// * `name` - the name of the driver
    ///
    /// ## Returns
    /// * `Driver` - the driver
    pub fn ensure_exists(conn: &mut PgConnection, name: &String) -> Driver {
        if !Driver::exists(conn, name) {
            Driver::new(conn, name)
        } else {
            Driver::get_by_name(conn, name).unwrap()
        }
    }

    /// # get the driver from lap
    /// get a driver from a lap. this function returns the driver that has driven the lap.
    ///
    /// ## Arguments
    /// * `connection` - the database connection
    /// * `lap` - the lap
    ///
    /// ## Returns
    /// * `Driver` - the driver
    pub fn from_lap(conn: &mut PgConnection, lap: Lap) -> Driver {
        Driver::get_by_id(conn, lap.driver)
    }

    /// # get the drivers for certain laps
    /// get the drivers for certain laps. this function returns the drivers that have driven the laps.
    ///
    /// ## Arguments
    /// * `connection` - the database connection
    /// * `laps` - the laps
    ///
    /// ## Returns
    /// * `Vec<Driver>` - the drivers
    pub fn from_laps(conn: &mut PgConnection, laps: &[Lap]) -> Vec<Driver> {
        use crate::schema::drivers::dsl::*;

        let driver_ids: Vec<i32> = laps.iter().map(|e| e.driver).collect();

        drivers
            .filter(id.eq_any(driver_ids))
            .load::<Driver>(conn)
            .unwrap()
    }

    pub fn from_single_lap_offline(all_drivers: &[Driver], lap: Lap) -> Option<Driver> {
        for driver in all_drivers {
            if driver.id == lap.id {
                return Some(driver.clone());
            }
        }

        None
    }

    pub fn from_laps_offline(all_drivers: &[Driver], laps: &[Lap]) -> Vec<Driver> {
        let drivers_map: HashMap<i32, Driver> = all_drivers
            .iter()
            .map(|e| (e.id, e.to_owned()))
            .collect();


        let mut drivers = HashSet::new();
        for lap in laps {
            drivers.insert(match drivers_map.get(&lap.id) {
                None => continue,
                Some(e) => e.to_owned()
            });
        }

        drivers.into_iter().collect()
    }

    /// # map drivers to laps
    /// map the passed in drivers to the passed in laps.
    /// the driver will be used as key and the laps will be the key
    ///
    /// ## Arguments
    /// * `drivers` - the drivers
    /// * `laps` - the laps
    ///
    /// ## returns
    /// * `HashMap<Driver, Vec<Lap>` - the drivers and laps
    pub fn map_to_laps(drivers: Vec<Driver>, laps: &[Lap]) -> HashMap<Driver, Vec<Lap>> {
        let drivers_map: HashMap<i32, Driver> = drivers.into_iter().as_map();

        let mut ret = HashMap::new();
        for lap in laps {
            let driver_in = drivers_map.get(&lap.driver).unwrap().to_owned();

            if let Entry::Vacant(e) = ret.entry(driver_in.clone()) {
                e.insert(vec![lap.clone()]);
            } else {
                ret.get_mut(&driver_in).unwrap().push(lap.clone());
            }
        }

        ret
    }

    /// # get the driver from multiple laps
    /// get the driver from multiple laps.
    /// the difference whith this fucntion and `from_laps` is that this function returns a hashmap
    /// with the driver as key and the laps as value.
    ///
    /// ## Arguments
    /// * `connection` - the database connection
    /// * `laps` - the laps
    ///
    /// ## Returns
    /// * `HashMap<Driver, Vec<Lap>>` - the drivers and their laps
    pub fn from_laps_into_map(conn: &mut PgConnection, laps: &[Lap]) -> HashMap<Driver, Vec<Lap>> {
        let drivers = Driver::from_laps(conn, laps);

        Driver::map_to_laps(drivers, laps)
    }

    /// convert the driver object to a NewDriver object
    ///
    /// ## Returns
    /// * `NewDriver` - the new driver object
    pub fn to_new(&self) -> NewDriver {
        NewDriver {
            name: self.name.clone(),
            rating: self.rating,
            uncertainty: self.uncertainty,
        }
    }

    /// # Get laps of a driver
    /// get all laps driven by a driver
    ///
    /// ## Arguments
    /// * `connection` - the database connection
    ///
    /// ## Returns
    /// * `Vec<Lap>` - the laps
    pub fn get_laps(&self, conn: &mut PgConnection) -> Vec<Lap> {
        use crate::schema::laps::dsl::*;
        laps.filter(driver.eq(self.id)).load::<Lap>(conn).unwrap()
    }

    /// # Get stats of a drivers stats for certain laps
    /// this function returns the stats of give laps only for the current driver
    ///
    /// ## Arguments
    /// * `laps` - the laps
    ///
    /// ## Returns
    /// * `DriverStats` - the stats
    pub fn get_stats_of_laps(&self, laps: &Vec<Lap>) -> DriverStats {
        let correct_laps = &laps
            .iter()
            .filter(|lap| lap.driver == self.id)
            .map(|e| e.to_owned())
            .collect();

        let lap_stats = Lap::get_stats_of_laps(correct_laps);
        let heat_count = Heat::amount_from_laps(correct_laps);

        DriverStats {
            name: self.name.clone(),
            fastest_lap_time: lap_stats.fastest_lap_time,
            avg_lap_time: lap_stats.avg_lap_time,
            median_lap_time: lap_stats.median_lap_time,
            total_laps: correct_laps.len() as i32,
            total_heats: heat_count as i32,
            rating: self.rating,
        }
    }

    /// # Get stats of a drivers stats for certain laps
    /// this function returns the stats of give laps only for the current driver
    /// this function is the same as `get_stats_of_laps` but it returns a different struct.
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `laps` - the laps
    ///
    /// ## Returns
    /// * `TemplateDataDriver` - the stats
    pub fn get_stats_for_laps(
        &self,
        conn: &mut PgConnection,
        laps: &Vec<Lap>,
    ) -> TemplateDataDriver {
        // get all laps that belong to this driver
        let mut laps_of_driver: Vec<TemplateDataLap> = Vec::new();
        let mut _lap_of_driver: &Lap = &Lap {
            id: 0,
            heat: 0,
            driver: 0,
            lap_in_heat: 0,
            lap_time: 0.0,
            kart_id: 0,
        };

        let mut fastest_lap: TemplateDataLap = TemplateDataLap {
            lap_in_heat: 0,
            lap_time: 20.0 * 60.0,
        };
        let mut total_lap_time: f64 = 0.0;

        for lap in laps {
            if lap.driver == self.id {
                total_lap_time += lap.lap_time;
                _lap_of_driver = lap;

                let lap_data = TemplateDataLap {
                    lap_in_heat: lap.lap_in_heat,
                    lap_time: lap.lap_time,
                };
                if fastest_lap.lap_time > lap.lap_time {
                    fastest_lap = lap_data.clone();
                }

                laps_of_driver.push(lap_data);
            }
        }

        let kart = Kart::get_by_id(conn, _lap_of_driver.kart_id);

        // separate the normal and abnormal laps
        let outliers: Vec<TemplateDataLap> = Lap::get_outlier_laps(&laps_of_driver);
        let normal_laps: Vec<TemplateDataLap> =
            Helpers::get_difference_between_vectors(&laps_of_driver, &outliers);

        TemplateDataDriver {
            driver_name: self.name.clone(),
            fastest_lap,
            all_laps: laps_of_driver.to_vec(),
            normal_laps: normal_laps.to_vec(),
            outlier_laps: outliers.to_vec(),
            total_laps: laps_of_driver.len(),
            avg_lap_time: Math::round_float_to_n_decimals(
                total_lap_time / laps_of_driver.len() as f64,
                3,
            ),
            kart: kart.number,
        }
    }

    /// # get the number of drivers
    /// get the number of drivers for the given laps
    ///
    /// ## Arguments
    /// * `laps` - the laps
    ///
    /// ## Returns
    /// * `usize` - the number of drivers
    pub fn count_from_laps(laps: &[Lap]) -> usize {
        let mut drivers_set: HashSet<i32> = HashSet::new();
        for lap in laps {
            drivers_set.insert(lap.driver);
        }

        drivers_set.len()
    }

    pub fn clear_cache(&self, rconn: &mut redis::Connection) {
        // get all keys
        // uri encode the name of the driver
        let encoded_name = self.name.replace(" ", "%20");
        let mut keys: HashSet<String> = rconn
            .keys::<String, Vec<String>>(format!("*{}*", encoded_name))
            .unwrap()
            .iter()
            .map(|e| e.to_owned())
            .collect();


        // searsh queries
        rconn.keys::<String, Vec<String>>("*/search/*".to_string())
            .unwrap()
            .iter()
            .for_each(|key| { keys.insert(key.to_owned()); });

        keys.insert("/api/drivers/all".to_string());
        keys.insert("/api/drivers/all/full".to_string());
        keys.insert("/api/heats/all".to_string());
        keys.insert("/api/heats/all/full".to_string());
        keys.insert("/api/heats/all".to_string());
        keys.insert("/api/heats/all/full".to_string());
        keys.insert("/drivers/all".to_string());

        // delete all keys
        for key in keys {
            rconn.del::<String, ()>(key).unwrap();
        }
    }

    /// # set the rating of a player to a new value
    /// this function sets the rating of a player to a new value
    /// the player that is being updated is the player whose id is given
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `driver_id` - the id of the driver
    /// * `new_rating` - the new rating
    pub fn set_rating_id(conn: &mut PgConnection, driver_id: i32, new_rating: WengLinRating) {
        diesel::update(drivers::table)
            .filter(drivers::id.eq(driver_id))
            .set((
                drivers::rating.eq(new_rating.rating),
                drivers::uncertainty.eq(new_rating.uncertainty),
            ))
            .execute(conn)
            .unwrap();
    }

    /// # set new skill ratings for the current player
    /// calls the fuction `set_rating_id` with the current driver
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `new_rating` - the new rating
    pub fn set_rating(&self, conn: &mut PgConnection, new_rating: WengLinRating) {
        Driver::set_rating_id(conn, self.id, new_rating);
    }

}

/// # sanitize name
/// sanitizes a name to be safe to store in the database
///
/// ## Arguments
/// * `name` - the name
///
/// ## Returns
/// * `String` - the sanitized name
pub fn sanitize_name(name: &str) -> String {
    let email_regex = Regex::new(r#"(?:[a-zA-Z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-zA-Z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)])"#).unwrap();
    let disallowed_chars = [
        '(', ')', '[', ']', '{', '}', '<', '>', ';', ':', ',', '/', '\\', '"', '`', '~', '!', '@',
        '#', '$', '%', '^', '&', '*', '+', '=', '?', '|', '_'
    ];

    let mut sanitized = name.trim().to_string();
    // remove emails
    sanitized = email_regex.replace_all(&sanitized, "").to_string();
    // remove disallowed chars
    sanitized = sanitized.replace(&disallowed_chars[..], "");
    sanitized = sanitized.trim_matches('-').to_string();

    sanitized.trim().to_lowercase().to_string()
}


#[derive(QueryableByName, Serialize, Deserialize, JsonResponse)]
pub struct DriverStats {
    #[diesel(sql_type = VarChar)]
    pub name: String,
    #[diesel(sql_type = Double)]
    pub fastest_lap_time: f64,
    #[diesel(sql_type = Double)]
    pub avg_lap_time: f64,
    #[diesel(sql_type = Double)]
    pub median_lap_time: f64,
    #[diesel(sql_type = Integer)]
    pub total_laps: i32,
    #[diesel(sql_type = Integer)]
    pub total_heats: i32,
    #[diesel(sql_type = Double)]
    pub rating: f64,
}
