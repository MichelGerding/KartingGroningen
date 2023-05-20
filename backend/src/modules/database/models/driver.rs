use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::modules::helpers::math::Math;
use crate::modules::database::models::session::Session;
use crate::modules::database::models::vehicle::Vehicle;
use crate::modules::database::models::lap::Lap;
use crate::modules::redis::Redis;
use crate::modules::traits::as_map::AsMap;
use crate::modules::traits::has_id::HasIdTrait;
use crate::{TemplateDataDriver, TemplateDataLap};

use identifiable_derive::HasId;
use regex::Regex;

use crate::macros::redis::{delete_keys};
use json_response_derive::JsonResponse;
use log::error;


use rocket::http::ContentType;
use rocket::response;
use rocket::response::Responder;
use rocket::response::Response;
use rocket::Request;
use skillratings::weng_lin::WengLinRating;

use crate::modules::database::models::general::{get_pool};

use crate::cornucopia::queries::driver::{create_driver, Driver as cDriver, DriverWithStats as cDriverWithStats, get_all_drivers, get_driver_by_id, get_driver_by_name, get_drivers_by_ids, get_driver_with_stats, search_driver_by_name, search_driver_with_stats_paginated, update_driver_rating};
use crate::modules::database::query::{get_vec as select_vec_from_db};
use crate::modules::database::query::get as select_from_db;
use crate::modules::traits::diff_between_vecs::GetDiff;


trait IdentifiableAsMap {
    fn get_id(&self) -> i32;
}

impl Into<Driver> for cDriver {
    fn into(self) -> Driver {
        Driver {
            id: self.id,
            name: self.name,
            rating: self.rating,
            uncertainty: self.uncertainty,
        }
    }
}

impl Into<DriverStats> for cDriverWithStats {
    fn into(self) -> DriverStats {
        DriverStats {
            name: self.name,
            fastest_lap_time: self.fastest_lap_time,
            avg_lap_time: self.avg_lap_time,
            median_lap_time: self.median_lap_time,
            total_laps: self.total_laps,
            total_heats: self.total_heats,
            rating: self.rating,
        }
    }
}


#[derive(Serialize, Debug, Clone, Deserialize, HasId)]
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
    /// * `Driver` - the driver that was created
    pub async fn new(name: &str) -> Driver {
        let client = &get_pool().get().await.unwrap();
        select_from_db!(create_driver(), client, &name, &25.0, &(25.0/3.0))
    }

    /// # Get all drivers
    /// get all drivers from the database
    ///
    /// ## Arguments
    /// * `connection` - the database connection
    ///
    /// ## Returns
    /// * `Vec<Driver> - all drivers
    pub async fn get_all() -> Vec<Driver> {
        let client = &get_pool().get().await.unwrap();
        select_vec_from_db!(get_all_drivers(), client)
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
    pub async fn get_by_id(id: i32) -> Driver {
        let client = &get_pool().get().await.unwrap();
        select_from_db!(get_driver_by_id(), client, &id)
    }

    pub async fn get_by_ids(ids: Vec<i32>) -> Vec<Driver> {
        let client = &get_pool().get().await.unwrap();
        select_vec_from_db!(get_drivers_by_ids(), client, &ids)
    }

    pub async fn search_with_stats(
        driver_name: String,
        page_size: u32,
        page: u32,
        _sort_col: String,
        _sort_dir: String,
    ) -> Vec<DriverStats> {
        // TODO:: re add ordering
        let client = &get_pool().get().await.unwrap();
        select_vec_from_db!(
            search_driver_with_stats_paginated(),
            client,
            &driver_name,
            &(page_size as i64),
            &((page * page_size) as i64))
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
    pub async fn get_all_with_stats() -> Vec<DriverStats> {
        let client = &get_pool().get().await.unwrap();

        select_vec_from_db!(
            search_driver_with_stats_paginated(),
            client,
            &"",
            &i64::MAX,
            &0)
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
    pub async fn get_driver_with_stats(driver_name: String) -> DriverStats {
        let client = &get_pool().get().await.unwrap();
        select_from_db!(
            search_driver_with_stats_paginated(),
            client,
            &driver_name,
            &1,
            &0)
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
    pub async fn exists(name: &String) -> bool {
        let client = &get_pool().get().await.unwrap();

        get_driver_by_name()
            .bind(client, name)
            .all()
            .await
            .unwrap()
            .len() > 0
    }

    /// # get a driver by name
    /// get a driver by name. if the driver does not exists it will panic
    ///
    /// ## Arguments
    /// * `connection` - the database connection
    ///
    /// ## Returns
    /// * `Driver` - the driver
    pub async fn get_by_name(name: &String) -> Driver {
        let client = &get_pool().get().await.unwrap();
        select_from_db!(get_driver_by_name(), client, name)
    }

    pub async fn search_by_name(
        name: &String,
        page: i32,
        page_size: i32,
    ) -> Vec<Driver>{
        let client = &get_pool().get().await.unwrap();
        select_vec_from_db!(
            search_driver_by_name(),
            client,
            &name,
            &(page as i64),
            &((page_size * page )as i64))
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
    pub async fn get_stats(&self) -> DriverStats {
        let client = &get_pool().get().await.unwrap();
        select_from_db!(get_driver_with_stats(), client, &self.id)
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
    pub async fn ensure_exists(name: &String) -> Driver {
        if !Driver::exists(name).await {
            Driver::new(name).await
        } else {
            Driver::get_by_name(name).await
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
    pub async fn from_lap(lap: Lap) -> Driver {
        Driver::get_by_id(lap.driver).await
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
    pub async fn from_laps(laps: &[Lap]) -> Vec<Driver> {
        let client = &get_pool().get().await.unwrap();

        let driver_ids: Vec<i32> = laps.iter().map(|e| e.driver).collect();
        select_vec_from_db!(get_drivers_by_ids(), client, &driver_ids)
    }


    /// # get the drivers of the laps
    /// get the drivers of the laps. this function returns the drivers that have driven the laps.
    /// this function does not require the connection a connection to the db. instead it requires
    /// a list of drivers
    ///
    /// if the driver is not found we return a empty vector
    ///
    /// ## Arguments
    /// * `all_drivers` - the drivers
    /// * `laps` - the laps
    ///
    /// ## Returns
    /// * `Vec<Driver>` - the drivers
    pub fn from_laps_offline(all_drivers: &[Driver], laps: &[Lap]) -> Vec<Driver> {
        let drivers_map: HashMap<i32, Driver> =
            all_drivers.iter().map(|e| (e.id, e.to_owned())).collect();

        let mut drivers = HashSet::new();
        for lap in laps {
            drivers.insert(match drivers_map.get(&lap.id) {
                None => continue,
                Some(e) => e.to_owned(),
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
    pub async fn from_laps_into_map(laps: &[Lap]) -> HashMap<Driver, Vec<Lap>> {
        Driver::map_to_laps(Driver::from_laps(laps).await, laps)
    }

    /// # Get laps of a driver
    /// get all laps driven by a driver
    ///
    /// ## Arguments
    /// * `connection` - the database connection
    ///
    /// ## Returns
    /// * `Vec<Lap>` - the laps
    pub async fn get_laps(&self) -> Vec<Lap> {
        Lap::from_driver(self).await
    }

    /// # Get stats of a drivers stats for certain laps
    /// this function returns the stats of give laps only for the current driver
    ///
    /// ## Arguments
    /// * `laps` - the laps
    ///
    /// ## Returns
    /// * `DriverStats` - the stats
    pub fn get_stats_of_laps(&self, laps: &Vec<Lap>) -> Option<DriverStats> {
        let correct_laps: Vec<Lap> = laps
            .iter()
            .filter(|lap| lap.driver == self.id)
            .map(|e| e.to_owned())
            .collect();

        if correct_laps.is_empty() {
            return None;
        }

        let lap_stats = Lap::get_stats_of_laps(&correct_laps);
        let heat_count = Session::amount_from_laps(&correct_laps);

        Some(DriverStats {
            name: self.name.clone(),
            fastest_lap_time: lap_stats.fastest_lap_time,
            avg_lap_time: lap_stats.avg_lap_time,
            median_lap_time: lap_stats.median_lap_time,
            total_laps: correct_laps.len() as i32,
            total_heats: heat_count as i32,
            rating: self.rating,
        })
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
    pub async fn get_stats_for_laps(
        &self,
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

        let kart = match Vehicle::get_by_id(_lap_of_driver.kart_id).await {
            Some(k) => k,
            None => {
                Vehicle {
                    id: -1,
                    number: -1,
                    brand: "".to_string(),
                    model: "".to_string(),
                    horsepower: 0,
                    modified: false,
                }
            }
        };

        // separate the normal and abnormal laps
        let outliers: Vec<TemplateDataLap> = Lap::get_outlier_laps(&laps_of_driver);
        let normal_laps: Vec<TemplateDataLap> = laps_of_driver.diff(&outliers);
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

    /// # clear the cache of the driver
    /// clear the cache of the driver
    ///
    /// ## Arguments
    /// * `rconn` - the redis connection
    pub fn clear_cache(&self, rconn: &mut redis::Connection) {
        // get all keys
        // uri encode the name of the driver
        let encoded_name = self.name.replace(" ", "%20");
        let mut keys: Vec<String> = match Redis::keys(rconn, encoded_name) {
            Ok(keys) => keys,
            Err(error) => {
                error!(target:"models/driver:clear_cache", "error while getting keys from redis: {}", error);
                return;
            }
        };

        // search queries
        match Redis::get_keys(rconn, "*/search/*") {
            Ok(keys_) => {
                keys_.iter().for_each(|key| {
                    keys.push(key.to_string());
                });
            }
            Err(error) => {
                error!(target:"models/driver:clear_cache", "error while getting search keys from redis: {}", error);
                return;
            }
        }

        keys.append(&mut vec![
            "/api/drivers/all".to_string(),
            "/api/drivers/all/full".to_string(),
            "/api/heats/all".to_string(),
            "/api/heats/all/full".to_string(),
            "/api/heats/all".to_string(),
            "/api/heats/all/full".to_string(),
            "/drivers/all".to_string(),
        ]);

        // delete all keys
        delete_keys!(rconn, keys, "models/driver:clear_cache");
    }

    /// # set the rating of a player to a new value
    /// this function sets the rating of a player to a new value
    /// the player that is being updated is the player whose id is given
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `driver_id` - the id of the driver
    /// * `new_rating` - the new rating
    pub async fn set_rating_id(
        driver_id: i32,
        new_rating: WengLinRating,
    ) -> u64 {
        let client = &get_pool().get().await.unwrap();

        update_driver_rating()
            .bind(client, &new_rating.rating, &new_rating.uncertainty, &driver_id)
            .one()
            .await
            .unwrap() as u64
    }

    /// # set new skill ratings for the current player
    /// calls the fuction `set_rating_id` with the current driver
    ///
    /// ## Arguments
    /// * `conn` - the database connection
    /// * `new_rating` - the new rating
    pub async fn set_rating(
        &self,
        new_rating: WengLinRating,
    ) -> u64 {
        Driver::set_rating_id(self.id, new_rating).await
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
        '#', '$', '%', '^', '&', '*', '+', '=', '?', '|', '_',
    ];

    let mut sanitized = name.trim().to_string();
    // remove emails
    sanitized = email_regex.replace_all(&sanitized, "").to_string();
    // remove disallowed chars
    sanitized = sanitized.replace(&disallowed_chars[..], "");
    sanitized = sanitized.trim_matches('-').to_string();

    sanitized.trim().to_lowercase().to_string()
}

#[derive(Serialize, Deserialize, JsonResponse)]
pub struct DriverStats {
    pub name: String,
    pub fastest_lap_time: f64,
    pub avg_lap_time: f64,
    pub median_lap_time: f64,
    pub total_laps: i32,
    pub total_heats: i32,
    pub rating: f64,
}
