use std::collections::{HashMap, HashSet};
use std::thread;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use identifiable_derive::HasId;
use log::error;
use serde::{Deserialize, Serialize};
use crate::macros::database_error_handeler::db_handle_get_error;
use crate::macros::redis::clear_cache;

use crate::modules::helpers::math::Math;
use crate::modules::models::driver::Driver;
use crate::modules::models::general::establish_connection;
use crate::modules::models::heat::Heat;
use crate::modules::models::kart::Kart;
use crate::modules::redis::Redis;
use crate::modules::traits::has_id::HasIdTrait;
use crate::schema::laps;
use crate::TemplateDataLap;

#[derive(Insertable, Serialize, Debug, Clone, Deserialize)]
#[diesel(table_name = laps)]
pub struct NewLap {
    pub heat: i32,
    pub driver: i32,
    pub lap_in_heat: i32,
    pub lap_time: f64,
    pub kart_id: i32,
}

#[derive(
Queryable, Serialize, Associations, Identifiable, PartialEq, Debug, Clone, Deserialize, HasId,
)]
#[diesel(belongs_to(Heat, foreign_key = heat))]
#[diesel(belongs_to(Driver, foreign_key = driver))]
#[diesel(belongs_to(Kart, foreign_key = kart_id))]
pub struct Lap {
    pub id: i32,
    pub heat: i32,
    pub driver: i32,
    pub lap_in_heat: i32,
    pub lap_time: f64,
    pub kart_id: i32,
}

impl Lap {
    /************ INSERTERS ************/
    /// # Insert a new lap into the database
    /// insert a new lap in the database from a NewLap object
    ///
    /// ## Arguments
    /// * `conn` - The database connection to use
    /// * `new_lap` - The new lap to insert
    ///
    /// ## Returns
    /// * `Lap` - The inserted lap
    pub fn new(conn: &mut PgConnection, new_lap: NewLap) -> QueryResult<Lap> {
        use crate::schema::laps::dsl::*;

        let lap: Lap = match diesel::insert_into(laps)
            .values(&new_lap)
            .get_result::<Lap>(conn) {
            Ok(lap) => lap,
            Err(error) => {
                error!(target:"models/lap:new", "Error inserting new lap: {}", error);
                return Err(error);
            }
        };

        let lap_driver = Driver::from_lap(conn, lap.clone());

        thread::spawn(move || {
            match lap_driver {
                Ok(driver_) => clear_cache!(driver_),
                Err(error) => {
                    error!(target: "models/lap:new", "Error clearing cache could not get driver: (error: {})", error);
                }
            };
        });

        Ok(lap)
    }

    /// # Insert a new lap into the database
    /// insert a new lap into the database by passing its values in directly
    ///
    /// ## Arguments
    /// * `conn` - The database connection to use
    /// * `heat_in` - The id of the heat the lap was driven in
    /// * `driver_in` - The id of the driver that drove the lap
    /// * `lap_in_heat_in` - The lap number in the heat
    /// * `lap_time_in` - The time of the lap
    /// * `kart_id_in` - The id of the kart driven in the lap
    ///
    /// ## Returns
    /// * `Lap` - The inserted lap
    pub fn insert_raw(
        conn: &mut PgConnection,
        heat_in: i32,
        driver_in: i32,
        lap_in_heat_in: i32,
        lap_time_in: f64,
        kart_id_in: i32,
    ) -> QueryResult<Lap> {
        Lap::new(conn, NewLap {
            heat: heat_in,
            driver: driver_in,
            lap_in_heat: lap_in_heat_in,
            lap_time: lap_time_in,
            kart_id: kart_id_in,
        })
    }

    /// # insert multiple laps into the database
    /// insert multiple laps into the database from a vector of NewLap objects
    ///
    /// ## Arguments
    /// * `conn` - The database connection to use
    /// * `new_laps` - The new laps to insert
    ///
    /// ## Returns
    /// * `Vec<Lap>` - The inserted laps
    pub fn insert_bulk(conn: &mut PgConnection, new_laps: &Vec<NewLap>) -> QueryResult<Vec<Lap>>{
        use crate::schema::laps::dsl::*;

        let inserted_laps = match diesel::insert_into(laps)
            .values(new_laps)
            .get_results::<Lap>(conn) {
            Ok(inserted_laps) => inserted_laps,
            Err(error) => {
                error!(target: "models/lap:insert_bulk", "Error inserting laps: (error: {})", error);
                return Err(error);
            }
        };


        let il = inserted_laps.clone();
        thread::spawn(move || {
            let db_conn = &mut establish_connection();
            let r_conn = &mut match Redis::connect() {
                Ok(rc) => rc,
                Err(error) => {
                    error!(target: "models/lap:insert_bulk", "Error connecting to redis: (error: {})", error);
                    return;
                }
            };

            // clear the cache for the involved drivers
            match Driver::from_laps(db_conn, &inserted_laps) {
                Ok(e) => {
                    e.iter()
                        .for_each(|e| e.clear_cache(r_conn));
                }
                Err(_) => {
                    error!(target:"models/lap:insert_bulk", "Error clearing cache could not get drivers");
                }
            }


            // clear for the heats
            match Heat::from_laps(db_conn, &inserted_laps) {
                Ok(v) => {
                    v.iter()
                        .for_each(|e| e.clear_cache(r_conn));
                }
                Err(error) => {
                    error!(target:"models/lap:insert_bulk", "Error clearing cache could not get heats: (error: {})", error);
                }
            }


            // clear for karts
            match Kart::from_laps(db_conn, &inserted_laps) {
                Ok(v) => {
                    v.iter()
                        .for_each(|e| e.clear_cache(r_conn));
                }
                Err(error) => {
                    error!(target:"models/lap:insert_bulk", "Error clearing cache could not get karts: (error: {})", error);
                }
            }

        });
        Ok(il)
    }

    /************ GETTERS ************/
    /// # Get a lap by its id
    /// get a lap from the database by its id
    ///
    /// ## Arguments
    /// * `conn` - The database connection to use
    /// * `id_in` - The id of the lap to get
    ///
    /// ## Returns
    /// * `Lap` - The lap with the given id
    pub fn from_id(conn: &mut PgConnection, id_in: i32) -> QueryResult<Lap> {
        use crate::schema::laps::dsl::*;

        laps.filter(id.eq(id_in))
            .first(conn)
    }

    /// # get all laps
    /// get all laps from the database
    ///
    /// ## Arguments
    /// * `conn` - The database connection to use
    ///
    /// ## Returns
    /// * `Vec<Lap>` - All laps in the database
    pub fn get_all(conn: &mut PgConnection) -> QueryResult<Vec<Lap>> {
        use crate::schema::laps::dsl::*;
        laps.load::<Lap>(conn)
    }

    /// # get all laps driven by a kart
    /// get all the laps driven by a kart from the database
    ///
    /// ## Arguments
    /// * `conn` - The database connection to use
    /// * `kart_in` - The id of the kart to get the laps for
    ///
    /// ## Returns
    /// * `Vec<Lap>` - All laps driven by the kart
    pub fn from_kart(conn: &mut PgConnection, kart_in: &Kart) -> QueryResult<Vec<Lap>> {
        Lap::from_karts(conn, &vec![kart_in.to_owned()])
    }

    pub fn from_kart_offline(all_laps: &[Lap], kart: &Kart) -> Vec<Lap> {
        all_laps
            .into_iter()
            .filter(|lap| lap.kart_id.eq(&kart.id))
            .map(|e| e.to_owned())
            .collect()
    }

    /// # get all laps driven by a list of karts
    /// get all the laps driven by a list of karts from the database
    ///
    /// ## Arguments
    /// * `conn` - The database connection to use
    /// * `karts_in` - The list of karts to get the laps for
    ///
    /// ## Returns
    /// * `Vec<Lap>` - All laps driven by the karts
    pub fn from_karts(conn: &mut PgConnection, karts_in: &[Kart]) -> QueryResult<Vec<Lap>> {
        use crate::schema::laps::dsl::*;
        laps.filter(kart_id.eq_any(karts_in.iter().map(|k| k.id).collect::<Vec<i32>>()))
            .load::<Lap>(conn)
    }

    /// # get all laps driven by a driver
    /// get all the laps driven by a driver from the database
    ///
    /// ## Arguments
    /// * `conn` - The database connection to use
    /// * `driver_in` - The id of the driver to get the laps for
    ///
    /// ## Returns
    /// * `Vec<Lap>` - All laps driven by the driver
    pub fn from_driver(conn: &mut PgConnection, driver_in: &Driver) -> QueryResult<Vec<Lap>> {
        Lap::from_drivers(conn, &vec![driver_in.to_owned()])
    }

    /// # get all laps driven by a list of drivers
    /// get all the laps driven by a list of drivers from the database
    ///
    /// ## Arguments
    /// * `conn` - The database connection to use
    /// * `drivers_in` - The list of drivers to get the laps for
    ///
    /// ## Returns
    /// * `Vec<Lap>` - All laps driven by the drivers
    pub fn from_drivers(conn: &mut PgConnection, drivers_in: &[Driver]) -> QueryResult<Vec<Lap>> {
        use crate::schema::laps::dsl::*;
        laps.filter(driver.eq_any(drivers_in.iter().map(|e| e.id)))
            .load::<Lap>(conn)
    }

    /// # get all laps driven by a list of drivers as map
    /// get all the laps driven by a list of drivers from the database
    /// using the driver as key
    ///
    /// ## Arguments
    /// * `conn` - The list of laps to match
    /// * `drivers_in` - The list of drivers to get the laps for
    ///
    /// ## Returns
    /// * `Hashmap<Driver, Vec<Lap>>` - All laps driven by the drivers
    pub fn from_drivers_as_map(
        conn: &mut PgConnection,
        drivers_in: &[Driver],
    ) -> QueryResult<HashMap<Driver, Vec<Lap>>> {
        let laps = match Lap::from_drivers(conn, drivers_in) {
            Ok(laps) => laps,
            Err(error) => {
                return Err(error);
            }
        };

        let mut heat_lap_map: HashMap<Driver, Vec<Lap>> = HashMap::new();
        for lap in laps {
            let driver_in = drivers_in
                .iter()
                .find(|d| d.id == lap.driver)
                .unwrap()
                .clone();

            if heat_lap_map.contains_key(&driver_in) {
                heat_lap_map.get_mut(&driver_in).unwrap().push(lap);
            } else {
                heat_lap_map.insert(driver_in, vec![lap]);
            }
        }

        Ok(heat_lap_map)
    }

    /// # get all laps driven in a heat
    /// get all the laps driven in a heat from the database
    ///
    /// ## Arguments
    /// * `conn` - The database connection to use
    /// * `heat_in` - The id of the heat to get the laps for
    ///
    /// ## Returns
    /// * `Vec<Lap>` - All laps driven in the heat
    pub fn from_heat(conn: &mut PgConnection, heat_in: &Heat) -> QueryResult<Vec<Lap>> {
        Lap::from_heats(conn, &vec![heat_in.to_owned()])
    }

    /// # get all laps driven in a list of heats
    /// get all the laps driven in a list of heats from the database
    ///
    /// ## Arguments
    /// * `conn` - The database connection to use
    /// * `heats_in` - The list of heats to get the laps for
    ///
    /// ## Returns
    /// * `Vec<Lap>` - All laps driven in the heats
    pub fn from_heats(conn: &mut PgConnection, heat_in: &[Heat]) -> QueryResult<Vec<Lap>> {
        use crate::schema::laps::dsl::*;
        laps.filter(heat.eq_any(heat_in.iter().map(|h| h.id)))
            .load::<Lap>(conn)
    }

    /// # get all laps from heats
    /// get all the laps driven in the passed heats from the database
    /// this is returned in a hashmap with the heat as key and laps as list
    ///
    /// ## Arguments
    /// * `conn` - The database connection to use
    /// * `heats_in` - The list of heats to get the laps for
    ///
    /// ## Returns
    /// * `HashMap<Heat, Vec<Lap>>` - All laps driven in the heats
    pub fn from_heats_as_map(
        conn: &mut PgConnection,
        heats_in: &[Heat],
    ) -> QueryResult<HashMap<Heat, Vec<Lap>>> {
        let laps = db_handle_get_error!(Lap::from_heats(conn, heats_in), "/models/lap:from_heats_as_map", "laps from heats");
        Ok(Lap::from_heats_as_map_offline(heats_in, &laps))
    }

    pub fn from_heats_as_map_offline(heats: &[Heat], laps: &[Lap]) -> HashMap<Heat, Vec<Lap>> {
        let mut heat_lap_map: HashMap<Heat, Vec<Lap>> = HashMap::new();
        for lap_ref in laps {
            let lap = lap_ref.to_owned();
            let heat_in = heats.iter().find(|h| h.id == lap.heat).unwrap().clone();

            if heat_lap_map.contains_key(&heat_in) {
                heat_lap_map.get_mut(&heat_in).unwrap().push(lap);
            } else {
                heat_lap_map.insert(heat_in, vec![lap]);
            }
        }

        heat_lap_map
    }

    /************ UTILS ************/

    /// # get the stats of the laps
    /// get the stats of the laps passed to the function
    ///
    /// ## Arguments
    /// * `laps` - The laps to get the stats for
    ///
    /// ## Returns
    /// * `LapStats` - The stats of the laps
    pub fn get_stats_of_laps(laps: &Vec<Lap>) -> LapsStats {
        let mut laps_time_sum: f64 = 0.0;
        let mut min_lap_time: f64 = f64::MAX;
        let mut laps_sorted: Vec<f64> = Vec::new();
        for lap in laps {
            laps_time_sum += lap.lap_time;
            if lap.lap_time < min_lap_time {
                min_lap_time = lap.lap_time;
            }

            laps_sorted.push(lap.lap_time);
        }

        laps_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = if laps_sorted.len() % 2 == 0 {
            (laps_sorted[laps_sorted.len() / 2 - 1] + laps_sorted[laps_sorted.len() / 2]) / 2.0
        } else {
            laps_sorted[laps_sorted.len() / 2]
        };

        LapsStats {
            avg_lap_time: laps_time_sum / laps.len() as f64,
            median_lap_time: median,
            fastest_lap_time: min_lap_time,
        }
    }

    /// # get the first lap that has a certain laptime
    /// if the laptime is not found we return None
    ///
    /// ## Arguments
    /// * `lap_time` - The laptime to search for
    /// * `laps` - The laps to search in
    ///
    /// ## Returns
    /// * `Option<Lap>` - The first lap that has the laptime
    pub fn find_laptime_in_laps(laptime: f64, laps: &[Lap]) -> Option<Lap> {
        for lap in laps {
            if lap.lap_time == laptime {
                return Some(lap.to_owned());
            }
        }
        return None;
    }

    /// # filter all laps that are slow or impossible fast
    /// filter all laps that are slow or impossible fast.
    /// this allows you to get a decent look at the consistency of the laps
    ///
    /// the cutoff is deteremend by the stanbdard deviation of the
    /// laps added onto the median laptime.
    /// a impossibly fast laptime is seen as a laptime faster then 45 seconds.
    /// this is deemed impossible because track record is a mid 46
    ///
    /// ## Arguments
    /// * `laps` - The laps to filter
    ///
    /// ## Returns
    /// * `Vec<Lap>` - The filtered laps
    pub fn filter_outliers(laps: &[Lap]) -> Vec<Lap> {
        let mut lap_times: Vec<f64> = Vec::new();
        for lap in laps.iter() {
            lap_times.push(lap.lap_time);
        }

        let standard_deviation = Math::standard_deviation(&lap_times);
        let center = Math::median(lap_times);

        let mut filtered: Vec<Lap> = Vec::new();
        for lap in laps.iter() {
            if lap.lap_time < center + standard_deviation && lap.lap_time > 45.0 {
                filtered.push(lap.clone());
            }
        }

        filtered
    }

    /// # get the standard deviation
    /// get the standard deviation of the laps passed to the function
    ///
    /// ## Arguments
    /// * `laps` - The laps to get the standard deviation for
    ///
    /// ## Returns
    /// * `f64` - The standard deviation of the laps
    pub fn get_standard_deviation_of_laps(laps: &[TemplateDataLap]) -> f64 {
        let laptimes: Vec<f64> = laps.iter().map(|lap| lap.lap_time).collect();
        Math::standard_deviation(&laptimes)
    }

    /// # get the median
    /// get the median laptime of the passed in laps
    ///
    /// ## Arguments
    /// * `laps` - The laps to get the median for
    ///
    /// ## Returns
    /// * `f64` - The median laptime of the laps
    pub fn get_median_laptime(laps: &[TemplateDataLap]) -> f64 {
        let mut sorted_laps = laps.to_owned();
        sorted_laps.sort_by(|a, b| a.lap_time.partial_cmp(&b.lap_time).unwrap());
        let middle = sorted_laps.len() / 2;
        if sorted_laps.len() % 2 == 0 {
            (sorted_laps[middle - 1].lap_time + sorted_laps[middle].lap_time) / 2.0
        } else {
            sorted_laps[middle].lap_time
        }
    }

    /// # get the average laptime
    /// get the average laptime of the passed in laps
    /// this is the sum of all the laptimes divided by the amount of laps
    ///
    /// ## Arguments
    /// * `laps` - The laps to get the average for
    ///
    /// ## Returns
    /// * `f64` - The average laptime of the laps
    pub fn get_mean_of_laps(laps: &Vec<TemplateDataLap>) -> f64 {
        let mut sum = 0.0;
        for lap in laps {
            sum += lap.lap_time;
        }
        sum / laps.len() as f64
    }

    /// # get the laps that are detemined to be outliers
    /// get the laps that are detemined to be outliers.
    /// this is deteremed the exact same way as in `filter_outliers`
    ///
    /// ## Arguments
    /// * `laps` - The laps to get the outliers for
    ///
    /// ## Returns
    /// * `Vec<TemplateDataLap>` - The outliers
    pub fn get_outlier_laps(laps: &Vec<TemplateDataLap>) -> Vec<TemplateDataLap> {
        // we expect all drivers to be the same so we only look at the lapstimes
        let mut outliers: Vec<TemplateDataLap> = Vec::new();

        // get the standard deviation of the laptimes in vec laps
        let mut lap_times: Vec<f64> = Vec::new();
        for lap in laps.iter() {
            lap_times.push(lap.lap_time);
        }
        let standard_deviation = Lap::get_standard_deviation_of_laps(laps);

        // get the center of the laptimes
        let center = Lap::get_mean_of_laps(laps);

        // get the outliers
        for lap in laps.iter() {
            if lap.lap_time > center + (standard_deviation * 2.0) || lap.lap_time < 45.0 {
                outliers.push(lap.clone());
            }
        }

        outliers
    }

    /// # get the amount of drivers
    /// get the amount of unique drivers in the laps
    pub fn get_amount_of_drivers_from_laps(laps: &[Lap]) -> usize {
        laps.iter()
            .map(|lap| lap.driver)
            .collect::<HashSet<i32>>()
            .len()
    }

}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LapsStats {
    pub avg_lap_time: f64,
    pub median_lap_time: f64,
    pub fastest_lap_time: f64,
}
