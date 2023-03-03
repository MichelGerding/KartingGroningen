use std::collections::{HashMap, HashSet};

use chrono::{NaiveDate, NaiveDateTime};
use diesel::dsl::exists;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_types::{Bool, Double, Integer, Text, Timestamp};
use diesel::{select, sql_query};
use diesel::result::Error;

use identifiable_derive::HasId;
use rocket::serde::Deserialize;
use serde::Serialize;

use crate::modules::helpers::math::Math;
use crate::modules::models::heat::Heat;
use crate::modules::models::lap::Lap;
use crate::schema::karts;

use rocket::response;
use rocket::Request;
use rocket::response::Responder;
use rocket::response::Response;
use rocket::http::ContentType;
use json_response_derive::JsonResponse;
use log::error;
use crate::macros::database_error_handeler::db_handle_get_error;
use crate::macros::redis::delete_keys;
use crate::modules::redis::Redis;


use crate::modules::traits::has_id::HasIdTrait;

#[derive(Insertable, Serialize, Debug, Clone, Deserialize)]
#[diesel(table_name = karts)]
pub struct NewKart {
    pub number: i32,
    pub is_child_kart: bool,
}

#[derive(Queryable, Serialize, Identifiable, PartialEq, Debug, Clone, Eq, Hash, HasId)]
pub struct Kart {
    pub id: i32,
    pub number: i32,
    pub is_child_kart: bool,
}

impl Kart {
    /********** INSERTERS **********/
    /// # insert a new lap into the database
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `number_in` - the number of the kart
    /// * `is_child_kart` - if the kart is a child kart
    ///
    /// ## Returns
    /// * `Kart` - the inserted kart
    pub fn new(conn: &mut PgConnection, number_in: i32, is_child_kart: Option<bool>) -> Kart {
        let new_kart = NewKart {
            number: number_in,
            is_child_kart: is_child_kart.unwrap_or(false),
        };

        diesel::insert_into(karts::table)
            .values(&new_kart)
            .get_result(conn)
            .expect("Error saving new kart")
    }

    /********** GETTERS **********/
    /// # get the kart by number
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `number_in` - the number of the kart
    ///
    /// ## Returns
    /// * `Kart` - the kart with the given number
    pub fn get_by_number(conn: &mut PgConnection, number_in: i32) -> Result<Kart, Error> {
        Ok(Kart::get_by_numbers(conn, &*vec![number_in])
            .into_iter()
            .next()
            .unwrap())

    }

    /// # get the karts by numbers
    /// get all karts corresponding to the given numbers
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `numbers_in` - the numbers of the karts
    ///
    /// ## Returns
    /// * `Vec<Kart>` - the karts with the given numbers
    pub fn get_by_numbers(conn: &mut PgConnection, numbers: &[i32]) -> Vec<Kart> {
        use crate::schema::karts::dsl::{karts, number};

        karts
            .filter(number.eq_any(numbers))
            .load::<Kart>(conn)
            .expect("Error loading karts")
    }

    /// # get kart from id
    /// get the kart corresponding to the given id
    /// the id is the databse id and not the number on the kart
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `id_in` - the id of the kart
    ///
    /// ## Returns
    /// * `Kart` - the kart with the given id
    pub fn get_by_id(conn: &mut PgConnection, id_in: i32) -> Kart {
        Kart::get_by_ids(conn, &*vec![id_in])
            .into_iter()
            .next()
            .unwrap()
    }

    /// # get karts from ids
    /// bulk version of get_by_id
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `ids_in` - the ids of the karts
    ///
    /// ## Returns
    /// * `Vec<Kart>` - the karts with the given ids
    pub fn get_by_ids(conn: &mut PgConnection, ids: &[i32]) -> Vec<Kart> {
        use crate::schema::karts::dsl::{id, karts};

        karts
            .filter(id.eq_any(ids))
            .load::<Kart>(conn)
            .expect("Error loading karts")
    }

    /// # get all karts
    /// get all karts in the database
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    ///
    /// ## Returns
    /// * `Vec<Kart>` - all karts in the database
    pub fn get_all(conn: &mut PgConnection) -> QueryResult<Vec<Kart>> {
        use crate::schema::karts::dsl::karts;

        karts.load::<Kart>(conn)
    }

    /// # get the kart of a lap
    /// get the kart a lap was driven by
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `lap` - the lap
    ///
    /// ## Returns
    /// * `Kart` - the kart of the lap
    pub fn from_lap(conn: &mut PgConnection, lap: Lap) -> QueryResult<Kart> {
        match Kart::from_laps(conn, &*vec![lap]) {
            Ok(karts) => {
                Ok(karts.into_iter()
                    .next()
                    .unwrap())
            }
            Err(error) => {
                error!("Error getting kart from lap: {}", error);
                Err(error)
            }
        }
    }

    /// # get the karts of laps
    /// bulk version of from_lap
    /// get the karts of the given laps
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `laps` - the laps
    ///
    /// ## Returns
    /// * `Vec<Kart>` - the karts of the laps
    pub fn from_laps(conn: &mut PgConnection, laps: &[Lap]) -> QueryResult<Vec<Kart>> {
        use crate::schema::karts::dsl::{id, karts};

        // get all unique kart ids
        let kart_ids = laps.iter().map(|e| e.kart_id).collect::<HashSet<i32>>();

        karts
            .filter(id.eq_any(kart_ids))
            .load::<Kart>(conn)
    }

    /// # get the karts of laps
    /// get the return only the karts that are in the give laps
    ///
    /// ## Arguments
    /// * `karts` - the karts to filter
    /// * `laps` - the laps
    ///
    /// ## Returns
    /// * `Vec<Kart>` - the karts of the laps
    pub fn from_laps_offline(karts: &[Kart], laps: &[Lap]) -> Vec<Kart> {
        let kart_ids: HashSet<i32> = laps.iter().map(|e| e.kart_id).collect();

        karts
            .iter()
            .filter(|e| kart_ids.contains(&e.id))
            .map(|e| e.to_owned())
            .collect()
    }

    /// # get laps per day
    /// get the laps driven by the kart for each day
    /// this is returned in a hashmap with the date as key and the laps as value
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    ///
    /// ## Returns
    /// * `HashMap<Date, Vec<Lap>>` - the laps per day
    pub fn get_laps_per_day(&self, conn: &mut PgConnection) -> QueryResult<HashMap<NaiveDate, Vec<Lap>>> {
        let v_laps = db_handle_get_error!(Lap::from_kart(conn, self), "/models/kart:get_laps_per_day", "laps from kart");

        let ids = v_laps.iter().map(|lap| lap.heat).collect::<Vec<i32>>();
        let heats = match Heat::get_from_db_ids(conn, &ids) {
            Ok(heats) => heats,
            Err(error) => {
                error!("Error getting heats: {}", error);
                return Err(error);
            }
        };

        let mut laps_per_day: HashMap<NaiveDate, Vec<Lap>> = HashMap::new();
        for lap in &v_laps {
            let heat = heats.iter().find(|heat| heat.id == lap.heat).unwrap();
            let date = heat.start_date.date();
            let laptime = lap.to_owned();

            if let std::collections::hash_map::Entry::Vacant(e) = laps_per_day.entry(date) {
                e.insert(vec![laptime]);
            } else {
                laps_per_day.get_mut(&date).unwrap().push(laptime);
            }
        }

        Ok(laps_per_day)
    }

    /// # get laptimes per day
    /// get the laptimes driven by the kart for each day
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    ///
    /// ## Returns
    /// * `HashMap<Date, Vec<f64>>` - the laptimes per day
    pub fn get_laptimes_per_day(&self, conn: &mut PgConnection) -> QueryResult<HashMap<NaiveDate, Vec<f64>>> {
        let laps_per_day = match self.get_laps_per_day(conn) {
            Ok(laps_per_day) => laps_per_day,
            Err(error) => {
                error!(target:"models/kart:get_lap_times_per_day", "Error getting laps per day: {}", error);
                return Err(error);
            }
        };

        let mut laptimes_per_day: HashMap<NaiveDate, Vec<f64>> = HashMap::new();
        for (date, laps) in laps_per_day {
            let laptimes = laps.iter().map(|lap| lap.lap_time).collect::<Vec<f64>>();
            laptimes_per_day.insert(date, laptimes);
        }

        Ok(laptimes_per_day)
    }

    /// # get the fastest laptime per day
    /// get the fastest laptime per day driven by the kart
    /// this is returned in a hashmap with the date as key and the laptime as value
    /// the fastest laptime is the fastest of all laps driven on that day
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    ///
    /// ## Returns
    /// * `HashMap<Date, f64>` - the fastest laptime per day
    pub fn get_minimum_laptime_per_day(&self, conn: &mut PgConnection) -> QueryResult<HashMap<NaiveDate, f64>> {
        let days = match self.get_laptimes_per_day(conn) {
            Ok(days) => days,
            Err(error) => {
                error!(target:"models/kart:get_minimum_laptime_per_day", "Error getting laptimes per day: {}", error);
                return Err(error);
            }
        };

        Ok(days.iter()
            .map(|(date, laps)| {
                let minimum = laps.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                (*date, Math::round_float_to_n_decimals(minimum, 2))
            })
            .collect())
    }

    /// # get the media laptime per day
    /// get the median laptime per day driven by the kart
    /// this is returned in a hashmap with the date as key and the laptime as value
    /// the median laptime is calculated by sorting the laptimes and taking the middle value
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    ///
    /// ## Returns
    /// * `HashMap<Date, f64>` - the median laptime per day
    pub fn get_median_laptime_per_day(&self, conn: &mut PgConnection) -> QueryResult<HashMap<NaiveDate, f64>> {
        let days = match self.get_laptimes_per_day(conn) {
            Ok(days) => days,
            Err(error) => {
                error!(target:"models/kart:get_median_laptime_per_day", "Error getting laptimes per day: {}", error);
                return Err(error);
            }
        };

        Ok(days.iter()
            .map(|(date, laps)| {
                let median = Math::median(laps.clone());
                (*date, Math::round_float_to_n_decimals(median, 2))
            })
            .collect())
    }

    /// # get the average laptime per day
    /// get the average laptime per day driven by the kart
    /// this is returned in a hashmap with the date as key and the laptime as value
    /// the average laptime is the average of all laps driven on that day
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    ///
    /// ## Returns
    /// * `HashMap<Date, f64>` - the average laptime per day
    pub fn get_average_laptime_per_day(&self, conn: &mut PgConnection) -> QueryResult<HashMap<NaiveDate, f64>> {
        let avg_day = match self.get_laptimes_per_day(conn) {
            Ok(avg_day) => avg_day,
            Err(error) => {
                error!(target:"models/kart:get_average_laptime_per_day", "Error getting laptimes per day: {}", error);
                return Err(error);
            }
        };

        Ok(avg_day
            .iter()
            .map(|(date, laps)| {
                let avg = laps.iter().sum::<f64>() / laps.len() as f64;
                (*date, Math::round_float_to_n_decimals(avg, 2))
            })
            .collect())
    }

    /// # get the stats of all karts per day
    /// get the stats of all karts per day
    /// this is returned in a hashmap with the kart as key and a vec of stats
    /// the stats are the fastest laptime, the median laptime, the average laptime and the date.
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    ///
    /// ## Returns
    /// * `HashMap<Kart, Vec<KartStatsPerDay>>` - the stats of all karts per day
    pub fn get_stats_per_day_from_db(
        conn: &mut PgConnection,
    ) -> HashMap<Kart, Vec<KartStatsPerDay>> {
        let kart_stats = sql_query(format!(
            "
            select
                k.id,
                k.number,
                k.is_child_kart,
                h.start_date,
                min(lap_time) as min_laptime,
                avg(lap_time) as avg_laptime,
                percentile_cont(0.5) WITHIN GROUP (ORDER BY lap_time) as median_laptime
            from karts k
            inner join laps l on k.id = l.kart_id
            inner join heats h on h.id = l.heat
            group by k.id, k.number, k.is_child_kart, h.start_date"
        ))
        .load::<KartStatsPerDay>(conn)
        .unwrap();

        let mut kart_stats_per_day: HashMap<Kart, Vec<KartStatsPerDay>> = HashMap::new();

        for stats in kart_stats {
            let kart = Kart {
                id: stats.id,
                number: stats.number,
                is_child_kart: false,
            };

            if let std::collections::hash_map::Entry::Vacant(e) =
                kart_stats_per_day.entry(kart.clone())
            {
                e.insert(vec![stats]);
            } else {
                kart_stats_per_day.get_mut(&kart).unwrap().push(stats);
            }
        }

        kart_stats_per_day
    }

    pub fn get_with_stats(conn: &mut PgConnection, kart_number: i32) -> Result<KartStats, Error> {
        sql_query(
            "
            select
                k.number,
                k.is_child_kart,
                CAST(count(l.id) AS INT) as lap_count,
                CAST(count(DISTINCT l.driver) AS INT) as driver_count
            from karts k
            inner join laps l on k.id = l.kart_id
            where k.number = $1
            group by k.id;"
        )
            .bind::<Integer, _>(kart_number)
            .get_result::<KartStats>(conn)
    }

    /// # get all karts and some basic info
    /// get the number, total laps, total drivers and if the kart is a child kart of all karts
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    ///
    /// ## Returns
    /// * `Vec<KartStats>` - the info of all karts
    pub fn get_all_with_stats(conn: &mut PgConnection, sort_col: String, sort_dir: String) -> Vec<KartStats> {
        sql_query(format!("
            select
                k.number,
                k.is_child_kart,
                CAST(count(l.id) AS INT) as lap_count,
                CAST(count(DISTINCT l.driver) AS INT) as driver_count
            from karts k
            inner join laps l on k.id = l.kart_id
            group by k.id
            order by {} {};", sort_col, sort_dir)
        )
            .bind::<Text, _>(sort_col)
            .bind::<Text, _>(sort_dir)
            .load::<KartStats>(conn)
            .unwrap()
    }

    /// # ensure kart exists
    /// ensure that the kart exists in the database
    /// if the kart does not exist, it will be created
    /// this is the prefered method to create a kart because it wont panic if the kart already exists
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `number` - the number of the kart
    /// * `is_child_kart` - if the kart is a child kart
    ///
    /// ## Returns
    /// * `Kart` - the kart
    pub fn ensure_exists(
        conn: &mut PgConnection,
        _number: i32,
        is_child_kart: Option<bool>,
    ) -> Kart {
        if !Kart::exists(conn, _number) {
            Kart::new(conn, _number, is_child_kart)
        } else {
            Kart::get_by_number(conn, _number).unwrap()
        }
    }

    /// # format the is_child_kart bool
    /// format the is_child_kart bool to a string to display
    ///
    /// ## Returns
    /// * `String` - the formatted string
    pub fn is_child_kart_to_string(&self) -> String {
        match self.is_child_kart {
            true => "Child kart".to_string(),
            false => "Adult kart".to_string(),
        }
    }

    /// # match given laps by the given karts
    /// this will store them in a hashmap with the kart as key and laps as value
    ///
    /// ## Arguments
    /// * `laps` - the laps to match
    /// * `karts` - a hashmap of karts with id as key
    ///
    /// ## Returns
    /// * `HashMap<Kart, Vec<Lap>>` - the laps matched by the karts
    pub fn map_laps_and_karts(laps: &[Lap], karts: HashMap<i32, Kart>) -> HashMap<Kart, Vec<Lap>> {
        let mut kart_laps: HashMap<Kart, Vec<Lap>> = HashMap::new();
        for lap in laps {
            let kart = karts.get(&lap.kart_id).unwrap();

            if let std::collections::hash_map::Entry::Vacant(e) = kart_laps.entry(kart.clone()) {
                e.insert(vec![lap.clone()]);
            } else {
                kart_laps.get_mut(kart).unwrap().push(lap.clone());
            }
        }

        kart_laps
    }

    /// # check if a kart existss
    /// check if a kart exists in the database
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `number` - the number of the kart
    ///
    /// ## Returns
    /// * `bool` - if the kart exists
    pub fn exists(conn: &mut PgConnection, number_in: i32) -> bool {
        use crate::schema::karts::dsl::{karts, number};

        select(exists(karts.filter(number.eq(number_in))))
            .get_result(conn)
            .unwrap()
    }

    pub fn clear_cache(&self, r_conn: &mut redis::Connection) {
        // get all keys
        let mut keys = match Redis::get_keys(r_conn, &self.number.to_string()) {
            Ok(keys) => keys,
            Err(error) => {
                error!(target:"model/kart:clear_cache", "Error getting keys: {}", error);
                return;
            }
        };

        keys.append(&mut vec![
            "/api/drivers/all".to_string(),
            "/api/drivers/all/full".to_string(),
            "/api/heats/all".to_string(),
            "/api/heats/all/full".to_string(),
            "/api/heats/all".to_string(),
            "/api/heats/all/full".to_string(),
            "/karts/all".to_string(),
        ]);

        delete_keys!(r_conn, keys, "models/kart:clear_cache");
    }
}

#[derive(QueryableByName, Serialize, Deserialize, JsonResponse)]
pub struct KartStats {
    #[diesel(sql_type = Integer)]
    pub number: i32,
    #[diesel(sql_type = Bool)]
    pub is_child_kart: bool,
    #[diesel(sql_type = Integer)]
    pub lap_count: i32,
    #[diesel(sql_type = Integer)]
    pub driver_count: i32,
}

#[derive(QueryableByName, Debug)]
pub struct KartStatsPerDay {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Integer)]
    pub number: i32,
    #[diesel(sql_type = Bool)]
    pub is_child_kart: bool,
    #[diesel(sql_type = Timestamp)]
    pub start_date: NaiveDateTime,
    #[diesel(sql_type = Double)]
    pub min_laptime: f64,
    #[diesel(sql_type = Double)]
    pub avg_laptime: f64,
    #[diesel(sql_type = Double)]
    pub median_laptime: f64,
}
