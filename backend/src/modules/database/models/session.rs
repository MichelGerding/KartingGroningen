use std::collections::{HashMap, HashSet};

use chrono::NaiveDateTime;
use identifiable_derive::HasId;
use serde::{Deserialize, Serialize};

use crate::modules::database::models::driver::Driver;
use crate::modules::database::models::vehicle::Vehicle;
use crate::modules::database::models::lap::{Lap, LapsStats};
use crate::modules::redis::Redis;
use crate::modules::traits::as_map::AsMap;
use crate::modules::traits::has_id::HasIdTrait;

use crate::macros::redis::{delete_keys};
use json_response_derive::JsonResponse;
use log::{error};

use rocket::http::ContentType;
use rocket::response;
use rocket::response::Responder;
use rocket::response::Response;
use rocket::Request;
use skillratings::weng_lin::{weng_lin_multi_team, WengLinConfig, WengLinRating};
use skillratings::MultiTeamOutcome;


use crate::modules::database::query::get_vec as select_vec_from_db;
use crate::modules::database::query::get as select_from_db;
use crate::modules::database::models::general::get_pool;
use crate::cornucopia::queries::heats::{Heat as cHeat, get_heat_with_stats_paginated, get_all_heats_with_stats, get_heat_with_stats, get_all_heats, get_heats_from_ids, get_heat_from_name, create_new_heat, delete_heat, get_all_chronologicaly};
use crate::cornucopia::queries::driver::{get_drivers_from_heat_in_order_fastest_lap, GetDriversFromHeatInOrderFastestLap};
use crate::cornucopia::queries::laps::delete_laps_from_heat;
use crate::modules::traits::primitive_to_naive_date::ChangeDateType;

impl Into<Session> for cHeat {
    fn into(self) -> Session {
        Session {
            id: self.id,
            heat_id: self.heat_id,
            heat_type: self.heat_type,
            start_date: self.start_date.to_naive_date(),
        }
    }
}


#[derive(Serialize, PartialEq, Debug, Clone, Deserialize, Eq, Hash, HasId)]
pub struct Session {
    pub id: i32,
    pub heat_id: String,
    pub heat_type: String,
    pub start_date: NaiveDateTime,
}

impl Session {
    /// # create heat
    /// create a new heat. this function panics if the heat already exists
    ///
    /// ## Arguments
    /// * `heat_id` - the heat id
    /// * `heat_type` - the heat type
    /// * `start_date` - the start date of the heat
    ///
    /// ## Returns
    /// * `Heat` - the created heat
    ///
    pub async fn new(
        heat_id: &str,
        heat_type: &str,
        start_date: &str,
    ) -> Session {
        let timestamp =
            NaiveDateTime::parse_from_str(start_date, "%Y-%m-%dT%H:%M:%S%.f%z").unwrap().to_primitive_date();

        let client = &get_pool().get().await.unwrap();
        select_from_db!(create_new_heat(),client, &heat_id, &heat_type, &timestamp)
    }

    /// # check if exists
    /// check if a heat exists
    ///
    /// ## Arguments
    /// * `heat_id` - the heat id
    ///
    /// ## Returns
    /// * `bool` - true if the heat exists
    pub async fn exists(heat_id: &str) -> bool {
        let client = &get_pool().get().await.unwrap();

        get_heat_from_name()
            .bind(client, &heat_id)
            .all()
            .await
            .unwrap()
            .len() > 0
    }

    /// # delete heat
    /// delete a heat
    ///
    /// ## Arguments
    pub async fn delete(&self) {
        Session::delete_db_id(self.id).await
    }

    /// # delete heat by id
    /// delete the heat with the given id
    /// the given id is the heat_id not the database id
    ///
    /// ## Arguments
    /// * `heat_id` - the id of the heat to delete
    pub async fn delete_id(heat_id: &str) {
        let heat = Session::get_by_id(heat_id).await;
        heat.delete().await;
    }

    /// # delete heat by db id
    /// delete the heat with the given database id
    /// the given id is the database id not the heat_id
    ///
    /// this function also deletes all the laps associated with the heat
    ///
    /// ## Arguments
    /// * `db_id` - the database id of the heat to delete
    pub async fn delete_db_id(db_id: i32) {
        let heat = Session::get_by_db_id(db_id).await;
        let laps = Lap::from_heat(&heat).await;
        let drivers = Driver::from_laps(&laps).await;

        let client = &get_pool().get().await.unwrap();

        delete_laps_from_heat()
            .bind(client, &heat.id)
            .await
            .unwrap();

        delete_heat()
            .bind(client, &heat.id)
            .one()
            .await
            .unwrap();

        task_queue::add_task(Box::new(move || {
            match &mut Redis::connect() {
                Ok(r_conn) => {
                    for driver in &drivers {
                        driver.clear_cache(r_conn);
                    }

                    heat.clear_cache(r_conn);
                }
                Err(err) => {
                    error!(target:"models/heat:delete_db_id", "Error connecting to redis: {}", err);
                }
            };
        })).unwrap();
    }

    pub fn clear_cache(&self, r_conn: &mut redis::Connection) {
        let mut keys: Vec<String> = match Redis::keys(r_conn, &self.heat_id) {
            Ok(keys) => keys,
            Err(error) => {
                error!(target:"models/driver:clear_cache", "error while getting keys from redis: {}", error);
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
            "/heats/all".to_string(),
        ]);

        // delete all keys
        delete_keys!(r_conn, keys, "models/heat:clear_cache");
    }

    /// # get heat by id
    /// get the heat with the given id
    /// the given id is the database id not the heat_id
    ///
    /// ## Arguments
    /// * `db_id` - the database id of the heat to get
    ///
    /// ## Returns
    /// * `Heat` - the heat
    pub async fn get_by_db_id(db_id: i32) -> Session {
        Session::get_from_db_ids(&[db_id]).await.pop().unwrap()
    }

    /// # get from db ids
    /// get the heats with the given database ids
    /// the given ids are the database ids not the heat_ids
    ///
    /// ## Arguments
    /// * `db_ids` - the database ids of the heats to get
    ///
    /// ## Returns
    /// * `Vec<Heat>` - the heats with the given database ids
    pub async fn get_from_db_ids(ids: &[i32]) -> Vec<Session> {
        let client = &get_pool().get().await.unwrap();
        
        select_vec_from_db!(get_heats_from_ids(), client, &ids)
    }

    /// # get the heats from a list of laps
    /// get the heats from a list of laps
    ///
    /// ## Arguments
    /// * `laps` - the laps
    ///
    /// ## Returns
    /// * `Vec<Heat>` - the heats
    pub async fn from_laps(laps: &[Lap]) -> Vec<Session> {
        let heat_ids = laps.iter().map(|e| e.heat).collect::<Vec<i32>>();

        Session::get_from_db_ids(&heat_ids).await
    }

    /// # get the heats from a list of laps
    /// get the heats from a list of laps
    ///
    /// ## Arguments
    /// * `laps` - the laps
    ///
    /// ## Returns
    /// * `Vec<Heat>` - the heats
    pub fn from_laps_offline(heats: &[Session], laps: &[Lap]) -> Vec<Session> {
        let heats_map: HashMap<i32, Session> = heats.iter().map(|e| (e.id, e.to_owned())).collect();

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
    /// * `heat_id_in` - the id of the heat to get
    ///
    /// ## Returns
    /// * `Heat` - the heat
    pub async fn get_by_id(heat_id: &str) -> Session {
        let client = &get_pool().get().await.unwrap();
        select_from_db!(get_heat_from_name(), client, &heat_id)
    }

    /// # get all heats
    ///
    /// ## Arguments
    ///
    /// ## Returns
    /// * `Vec<Heat>` - all the heats
    pub async fn get_all() -> Vec<Session> {
        let client = &get_pool().get().await.unwrap();
        select_vec_from_db!(get_all_heats(), client)
    }

    /// # get the laps of the heat
    /// get the laps of the heat
    ///
    /// ## Arguments
    ///
    /// ## Returns
    /// * `Vec<Lap>` - the laps of the heat
    pub async fn get_laps(&self) -> Vec<Lap> {
        Lap::from_heat(self).await
    }

    /// # get all heats with stats
    /// get all heats with basic stats.
    /// the stats given are the number of laps and the number of drivers
    ///
    /// ## Arguments
    ///
    /// ## Returns
    /// * `Vec<HeatStats>` - all the heats with stats
    pub async fn get_all_with_stats() -> Vec<HeatStats> {
       let client= &get_pool().get().await.unwrap();
        select_vec_from_db!(get_all_heats_with_stats(), client )
    }
    // q: String, page: Option<i64>, page_size: Option<i64>

    pub async fn get_all_paginated(
        page: i64,
        page_size: i64,
        _sort_dir: String,
        _sort_col: String,
    ) -> Vec<HeatStats> {
        let client = &get_pool().get().await.unwrap();
        select_vec_from_db!(
            get_heat_with_stats_paginated(),
            client,
            &page_size,
            &(page_size*page)
        )
    }

    /// # get a single heat with stats
    /// get a single heat with the basic stats: lap count, driver count,
    /// fastest lap, and average lap
    ///
    /// ## Arguments
    /// * `heat_id` - the id of heat to search
    ///
    /// ## Returns
    /// * `HeatStats` - heat and its stats
    pub async fn get_with_stats(heat_id: String) -> HeatStats {
        let client = &get_pool().get().await.unwrap();
        select_from_db!(get_heat_with_stats(), client, &heat_id)
    }

    /// # get all heats sorted by date
    ///
    /// ## Arguments
    ///
    /// ## Returns
    /// * `Vec<Heat>` - all the heats sorted by date
    pub async fn get_all_chronologicaly() -> Vec<Session> {
        let client = &get_pool().get().await.unwrap();
        select_vec_from_db!(get_all_chronologicaly(), client)
    }

    /// # ensure a heat exists
    /// ensure a heat exists
    /// if the heat does not exist it will be created
    /// this function is prefered over `new` because it wont panic
    /// if the heat already exists
    ///
    /// ## Arguments
    /// * `heat_id` - the id of the heat
    /// * `heat_type` - the type of the heat
    /// * `start_date` - the start date of the heat
    ///
    /// ## Returns
    /// * `Heat` - the heat
    pub async fn ensure_exists(
        heat_id: &str,
        heat_type: &str,
        start_time: &str,
    ) -> Session {
        if !Session::exists(heat_id).await {
            Session::new(heat_id, heat_type, start_time).await
        } else {
           Session::get_by_id(heat_id).await
        }
    }


    /// # get laps per driver
    /// get all laps driven by each driver in the heat
    /// the function returns a hashmap that uses the driver as key and laps as value
    ///
    /// ## Arguments
    ///
    /// ## Returns
    /// * `HashMap<String, Vec<Lap>>` - the laps per driver
    pub async fn laps_per_driver(&self) -> HashMap<Driver, Vec<Lap>> {
        let v_laps = Lap::from_heat(self).await;
        let drivers = Driver::from_laps(&v_laps).await;

        Session::parse_laps_and_drivers_into_map(&v_laps, &drivers)
    }

    /// # get driver stats
    /// get the stats of all drivers in the heat
    /// the function returns a hashmap that uses the driver as key and the stats as value
    pub async fn get_driver_stats(&self) -> HashMap<Driver, LapsStats> {
        let laps_per_driver = self.laps_per_driver().await;
        let mut driver_stats = HashMap::new();
        for (driver, laps) in laps_per_driver {
            driver_stats
                .insert(driver.to_owned(), Lap::get_stats_of_laps(&laps.to_owned()));
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

    pub async fn get_full_info(&self) -> FullHeatInfo {

        let laps = Lap::from_heat(self).await;
        let drivers: Vec<Driver> = Driver::from_laps(&laps).await;
        let karts: Vec<Vehicle> = Vehicle::from_laps(&laps).await;

        if karts.is_empty() {
            error!(target:"models/heat:get_full_info", "Error getting karts from laps");
            panic!("models/heat:get_full_info >> Error getting karts from laps")
        };

        let laps_per_driver = Session::parse_laps_and_drivers_into_map(&laps, &drivers);

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
    pub fn get_laps_per_heat(heats: &[Session], laps: &[Lap]) -> HashMap<Session, Vec<Lap>> {
        let heat_map = heats.to_owned().into_iter().as_map();

        let mut heat_laps_map = HashMap::new();
        for lap_reference in laps {
            let lap = lap_reference.to_owned();
            let heat: Session = heat_map.get(&lap.heat).unwrap().to_owned();

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

    pub async fn apply_ratings(&self) {
        let client = &get_pool().get().await.unwrap();

        // get the order the drivers finished in the heat
        let drivers: Vec<GetDriversFromHeatInOrderFastestLap> = select_vec_from_db!(get_drivers_from_heat_in_order_fastest_lap(), client, &self.id);

        let teams: Vec<Vec<WengLinRating>> = drivers
            .iter()
            .map(|driver| {
                return vec![WengLinRating {
                    rating: driver.rating,
                    uncertainty: driver.uncertainty,
                }];
            })
            .collect();

        // create the ratingh groups
        let mut rating_groups = Vec::new();
        for (position, _) in drivers.iter().enumerate() {
            let result = MultiTeamOutcome::new(position + 1);

            rating_groups.push((&(teams[position])[..], result));
        }

        let new_ratings = weng_lin_multi_team(&rating_groups[..], &WengLinConfig::default());
        for (position, driver) in drivers.iter().enumerate() {
            let new_rating = &new_ratings[position];
            Driver::set_rating_id(driver.id, new_rating[0]).await;
        }
    }
}

#[derive(Serialize, Deserialize, JsonResponse)]
pub struct HeatStats {
    pub heat_id: String,
    pub heat_type: String,
    pub start_time: NaiveDateTime,
    pub amount_of_laps: i32,
    pub amount_of_drivers: i32,
    pub fastest_lap_time: f64,
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
    pub kart: Vehicle,
    pub laps: Vec<Lap>,
}

impl Into<HeatStats> for crate::cornucopia::queries::heats::GetHeatWithStats {
    fn into(self) -> HeatStats {
        HeatStats {
            heat_id: self.heat_id,
            heat_type: self.heat_type,
            start_time: self.start_time.to_naive_date(),
            amount_of_laps: self.amount_of_laps,
            amount_of_drivers: self.amount_of_drivers,
            fastest_lap_time: self.fastest_lap_time,
            average_lap_time: self.average_lap_time,
        }
    }
}