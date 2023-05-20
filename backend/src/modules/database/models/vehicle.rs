use std::collections::{HashMap, HashSet};

use chrono::{NaiveDate, NaiveDateTime};

use identifiable_derive::HasId;
use rocket::serde::Deserialize;
use serde::Serialize;

use crate::modules::helpers::math::Math;
use crate::modules::database::models::session::Session;
use crate::modules::database::models::lap::Lap;

use crate::macros::redis::delete_keys;
use crate::modules::redis::Redis;
use json_response_derive::JsonResponse;
use log::error;

use rocket::http::ContentType;
use rocket::response;
use rocket::response::Responder;
use rocket::response::Response;
use rocket::Request;



use crate::modules::traits::has_id::HasIdTrait;

use crate::cornucopia::queries::kart::{create_kart, get_all_karts, get_all_karts_with_stats, get_kart_by_id, get_kart_by_number, get_kart_from_lap, get_kart_with_stats, get_karts_by_ids, get_karts_by_numbers, get_karts_from_laps, get_karts_stats_per_day};
use crate::modules::traits::primitive_to_naive_date::ChangeDateType;
use crate::modules::database::models::general::{get_pool};

#[derive(Serialize, PartialEq, Debug, Clone, Eq, Hash, HasId)]
pub struct Vehicle {
    pub id: i32,
    pub number: i32,
    pub brand: String,
    pub model: String,
    pub horsepower: i32,
    pub modified: bool,
}

impl Vehicle {
    /********** INSERTERS **********/
    /// # insert a new lap into the database
    ///
    /// ## Arguments
    /// * `number_in` - the number of the kart
    ///
    /// ## Returns
    /// * `Kart` - the inserted kart
    pub async fn new(
        number: i32,
        brand: String,
        model: String,
        horsepower: i32,
        modified: bool) -> Vehicle {

        let client = get_pool().get().await.unwrap();

        let id = match create_kart()
            .bind(&client, &number, &brand, &model, &horsepower, &modified)
            .one().await {
            Ok(id) => id,
            Err(error) => {
                panic!("error getting kart {:#?}", error)
            }
        };

        return Vehicle {
            id,
            number,
            brand,
            model,
            horsepower,
            modified,
        }

    }

    /********** GETTERS **********/
    /// # get the kart by number
    ///
    /// ## Arguments
    /// * `number_in` - the number of the kart
    ///
    /// ## Returns
    /// * `Kart` - the kart with the given number
    pub async fn get_by_number(number: i32) -> Option<Vehicle> {
        let client = get_pool().get().await.unwrap();


        match get_kart_by_number()
            .bind(&client, &number)
            .one().await {
            Ok(k) => Some(k.into()),
            Err(error) => {
                println!("{}", error);
                None
            }
        }
    }

    /// # get the karts by numbers
    /// get all karts corresponding to the given numbers
    ///
    /// ## Arguments
    /// * `numbers_in` - the numbers of the karts
    ///
    /// ## Returns
    /// * `Vec<Kart>` - the karts with the given numbers
    pub async fn get_by_numbers(numbers: Vec<i32>) -> Vec<Vehicle> {
        let client = get_pool().get().await.unwrap();

        match get_karts_by_numbers()
            .bind(&client, &numbers)
            .all().await {
            Ok(karts) => {
                karts.into_iter().map(|k| k.into()).collect()
            }
            Err(error) => {
                println!("{}", error);
                vec![]
            }
        }
    }

    /// # get kart from id
    /// get the kart corresponding to the given id
    /// the id is the databse id and not the number on the kart
    ///
    /// ## Arguments
    /// * `id` - the id of the kart
    ///
    /// ## Returns
    /// * `Kart` - the kart with the given id
    pub async fn get_by_id(id: i32) -> Option<Vehicle> {
        let client = get_pool().get().await.unwrap();

        match get_kart_by_id()
            .bind(&client, &id)
            .one().await {
            Ok(k) => Some(k.into()),
            Err(error) => {
                println!("{}", error);
                None
            }
        }
    }

    /// # get karts from ids
    /// bulk version of get_by_id
    ///
    /// ## Arguments
    /// * `ids_in` - the ids of the karts
    ///
    /// ## Returns
    /// * `Vec<Kart>` - the karts with the given ids
    pub async fn get_by_ids(ids: &[i32]) -> Vec<Vehicle> {
        let client = get_pool().get().await.unwrap();

        match get_karts_by_ids()
            .bind(&client, &ids)
            .all().await {
            Ok(karts) => {
                karts.into_iter().map(|k| k.into()).collect()
            }
            Err(error) => {
                println!("{}", error);
                vec![]
            }
        }
    }

    /// # get all karts
    /// get all karts in the database
    ///
    /// ## Arguments
    ///
    /// ## Returns
    /// * `Vec<Kart>` - all karts in the database
    pub async fn get_all() -> Vec<Vehicle> {
        let client = get_pool().get().await.unwrap();

        match get_all_karts()
            .bind(&client)
            .all().await {
            Ok(karts) => {
                return karts.into_iter().map(|k| k.into()).collect();
            }
            Err(error) => {
                println!("{}", error);
                return vec![];
            }
        }
    }

    /// # get the kart of a lap
    /// get the kart a lap was driven by
    ///
    /// ## Arguments
    /// * `lap` - the lap
    ///
    /// ## Returns
    /// * `Kart` - the kart of the lap
    pub async fn from_lap(lap: Lap) -> Option<Vehicle> {
        let client = get_pool().get().await.unwrap();

        match get_kart_from_lap()
            .bind(&client, &lap.id)
            .one().await {
            Ok(k) => {
                return Some(k.into());
            }
            Err(error) => {
                println!("{}", error);
                return None;
            }
        }
    }

    /// # get the karts of laps
    /// bulk version of from_lap
    /// get the karts of the given laps
    ///
    /// ## Arguments
    /// * `laps` - the laps
    ///
    /// ## Returns
    /// * `Vec<Kart>` - the karts of the laps
    pub async fn from_laps(laps: &[Lap]) -> Vec<Vehicle> {
        let client = get_pool().get().await.unwrap();


        match get_karts_from_laps()
            .bind(&client, &laps.into_iter().map(|l| l.into()).collect::<Vec<i32>>())
            .all().await {
            Ok(laps) => {
                return laps.into_iter().map(|k| k.into()).collect();
            }
            Err(error) => {
                println!("{}", error);
                return vec![];
            }
        }
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
    pub fn from_laps_offline(karts: &[Vehicle], laps: &[Lap]) -> Vec<Vehicle> {
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
    ///
    /// ## Returns
    /// * `HashMap<Date, Vec<Lap>>` - the laps per day
    pub async fn get_laps_per_day(&self) -> HashMap<NaiveDate, Vec<Lap>> {
        let v_laps = Lap::from_kart(self).await;

        let ids = v_laps.iter().map(|lap: &Lap| lap.heat).collect::<Vec<i32>>();
        let heats: Vec<Session> = Session::get_from_db_ids(&ids).await;

        let mut laps_per_day: HashMap<NaiveDate, Vec<Lap>> = HashMap::new();
        for lap in &v_laps {
            let heat = heats.iter().find(|heat | heat.id == lap.heat).unwrap();
            let date = heat.start_date.date();
            let laptime = lap.to_owned();

            if let std::collections::hash_map::Entry::Vacant(e) = laps_per_day.entry(date) {
                e.insert(vec![laptime]);
            } else {
                laps_per_day.get_mut(&date).unwrap().push(laptime);
            }
        }

        laps_per_day
    }

    /// # get laptimes per day
    /// get the laptimes driven by the kart for each day
    ///
    /// ## Arguments
    ///
    /// ## Returns
    /// * `HashMap<Date, Vec<f64>>` - the laptimes per day
    pub async fn get_laptimes_per_day(&self) -> HashMap<NaiveDate, Vec<f64>> {
        let laps_per_day = self.get_laps_per_day().await;

        let mut laptimes_per_day: HashMap<NaiveDate, Vec<f64>> = HashMap::new();
        for (date, laps) in laps_per_day {
            let laptimes = laps.iter().map(|lap| lap.lap_time).collect::<Vec<f64>>();
            laptimes_per_day.insert(date, laptimes);
        }

        laptimes_per_day
    }

    /// # get the fastest laptime per day
    /// get the fastest laptime per day driven by the kart
    /// this is returned in a hashmap with the date as key and the laptime as value
    /// the fastest laptime is the fastest of all laps driven on that day
    ///
    /// ## Arguments
    ///
    /// ## Returns
    /// * `HashMap<Date, f64>` - the fastest laptime per day
    pub async fn get_minimum_laptime_per_day(&self) -> HashMap<NaiveDate, f64> {
        let days = self.get_laptimes_per_day().await;

        days
            .iter()
            .map(|(date, laps)| {
                let minimum = laps.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                (*date, Math::round_float_to_n_decimals(minimum, 2))
            })
            .collect()
    }

    /// # get the media laptime per day
    /// get the median laptime per day driven by the kart
    /// this is returned in a hashmap with the date as key and the laptime as value
    /// the median laptime is calculated by sorting the laptimes and taking the middle value
    ///
    /// ## Arguments
    ///
    /// ## Returns
    /// * `HashMap<Date, f64>` - the median laptime per day
    pub async fn get_median_laptime_per_day(&self) -> HashMap<NaiveDate, f64> {
        let days = self.get_laptimes_per_day().await;

        days
            .iter()
            .map(|(date, laps)| {
                let median = Math::median(laps.clone());
                (*date, Math::round_float_to_n_decimals(median, 2))
            })
            .collect()
    }

    /// # get the average laptime per day
    /// get the average laptime per day driven by the kart
    /// this is returned in a hashmap with the date as key and the laptime as value
    /// the average laptime is the average of all laps driven on that day
    ///
    /// ## Arguments
    ///
    /// ## Returns
    /// * `HashMap<Date, f64>` - the average laptime per day
    pub async fn get_average_laptime_per_day(&self) -> HashMap<NaiveDate, f64> {
        let avg_day = self.get_laptimes_per_day().await ;

        avg_day
            .iter()
            .map(|(date, laps)| {
                let avg = laps.iter().sum::<f64>() / laps.len() as f64;
                (*date, Math::round_float_to_n_decimals(avg, 2))
            })
            .collect()
    }

    /// # get the stats of all karts per day
    /// get the stats of all karts per day
    /// this is returned in a hashmap with the kart as key and a vec of stats
    /// the stats are the fastest laptime, the median laptime, the average laptime and the date.
    ///
    /// ## Arguments
    ///
    /// ## Returns
    /// * `HashMap<Kart, Vec<KartStatsPerDay>>` - the stats of all karts per day
    pub async fn get_stats_per_day_from_db() -> HashMap<Vehicle, Vec<KartStatsPerDay>> {
        let client = get_pool().get().await.unwrap();

        let kart_stats: Vec<KartStatsPerDay> = match get_karts_stats_per_day()
            .bind(&client)
            .all().await {
            Ok(stats) => {
                stats.into_iter().map(|s| s.into()).collect()
            }
            Err(error) => {
                println!("{}", error);
                vec![]
            }
        };

        let mut kart_stats_per_day: HashMap<Vehicle, Vec<KartStatsPerDay>> = HashMap::new();

        for stats in kart_stats {
            let kart = Vehicle {
                id: stats.id,
                number: stats.number,
                brand: "".to_string(),
                model: "".to_string(),
                horsepower: 0,
                modified: false,
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

    pub async fn get_with_stats(kart_number: i32) -> Option<KartStats> {
        let client = get_pool().get().await.unwrap();

        return match get_kart_with_stats()
            .bind(&client, &kart_number)
            .one().await {
            Ok(stat) => Some(stat.into()),
            Err(error) => {
                println!("{}", error);
                None
            }
        };
    }

    /// # get all karts and some basic info
    /// get the number, total laps, total drivers and if the kart is a child kart of all karts
    ///
    /// ## Arguments
    ///
    /// ## Returns
    /// * `Vec<KartStats>` - the info of all karts
    pub async fn get_all_with_stats(
        sort_col: String,
        _sort_dir: String,
    ) -> Vec<KartStats> {
        let client = get_pool().get().await.unwrap();

        match get_all_karts_with_stats()
            .bind(&client, &sort_col)
            .all().await {
            Ok(stats) => {
                stats.into_iter().map(|s| s.into()).collect()
            }
            Err(error) => {
                println!("{}", error);
                vec![]
            }
        }
    }

    /// # ensure kart exists
    /// ensure that the kart exists in the database
    /// if the kart does not exist, it will be created
    /// this is the prefered method to create a kart because it wont panic if the kart already exists
    ///
    /// ## Arguments
    /// * `number` - the number of the kart
    /// * `is_child_kart` - if the kart is a child kart
    ///
    /// ## Returns
    /// * `Kart` - the kart
    pub async fn ensure_exists(
        number: i32,
        brand: String,
        model: String,
        horsepower: i32,
        modified: bool
    ) -> Vehicle {
        if !Vehicle::exists(number).await {
            Vehicle::new(
                number,
                brand, model, horsepower, modified).await
        } else {
            Vehicle::get_by_number(number).await.unwrap()
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
    pub fn map_laps_and_karts(laps: &[Lap], karts: HashMap<i32, Vehicle>) -> HashMap<Vehicle, Vec<Lap>> {
        let mut kart_laps: HashMap<Vehicle, Vec<Lap>> = HashMap::new();
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
    /// * `number` - the number of the kart
    ///
    /// ## Returns
    /// * `bool` - if the kart exists
    pub async fn exists(number: i32) -> bool {
        let client = &get_pool().get().await.unwrap();

        get_kart_by_number()
            .bind(client, &number)
            .all()
            .await
            .unwrap()
            .len() > 0
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

#[derive(Serialize, Deserialize, JsonResponse)]
pub struct KartStats {
    pub number: i32,
    pub brand: String,
    pub model: String,
    pub horsepower: i32,
    pub modified: bool,
    pub lap_count: i32,
    pub driver_count: i32,
}

#[derive(Debug)]
pub struct KartStatsPerDay {
    pub id: i32,
    pub number: i32,
    pub brand: String,
    pub model: String,
    pub horsepower: i32,
    pub modified: bool,    pub start_date: NaiveDateTime,
    pub min_laptime: f64,
    pub avg_laptime: f64,
    pub median_laptime: f64,
}


impl Into<Vehicle> for crate::cornucopia::queries::kart::Kart {
    fn into(self) -> Vehicle {
        return Vehicle {
            id: self.id,
            number: self.number,
            brand: "".to_string(),
            model: "".to_string(),
            horsepower: 0,
            modified: false,
        }
    }
}

impl Into<KartStatsPerDay> for crate::cornucopia::queries::kart::GetKartsStatsPerDay {
    fn into(self) -> KartStatsPerDay {
        KartStatsPerDay {
            id: self.id,
            number: self.number,
            brand: "".to_string(),
            model: "".to_string(),
            horsepower: 0,
            modified: false,
            start_date: self.start_date.to_naive_date(),
            min_laptime: self.min_laptime,
            avg_laptime: self.avg_laptime,
            median_laptime: self.median_laptime,
        }
    }
}
impl Into<KartStats> for crate::cornucopia::queries::kart::KartWithStats {
    fn into(self) -> KartStats {
        KartStats {
            number: self.number,
            brand: "".to_string(),
            model: "".to_string(),
            horsepower: 0,
            modified: false,
            lap_count: self.lap_count,
            driver_count: self.driver_count,
        }
    }
}