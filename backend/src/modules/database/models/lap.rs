use std::collections::{HashMap};


use identifiable_derive::HasId;
use serde::{Deserialize, Serialize};

use crate::cornucopia::queries::laps::{get_lap, get_laps_from_driver, insert_laps_bulk, get_laps_from_kart, get_laps_from_drivers, get_laps_from_heat, get_laps_from_heats};

use crate::modules::helpers::math::Math;
use crate::modules::database::models::driver::Driver;
use crate::modules::database::models::general::get_pool;
use crate::modules::database::models::session::Session;
use crate::modules::database::models::vehicle::Vehicle;
use crate::modules::traits::has_id::HasIdTrait;
use crate::TemplateDataLap;

use crate::modules::database::query::get_vec as select_vec_from_db;
use crate::modules::database::query::get as select_from_db;


#[derive( Serialize, Debug, Clone, Deserialize)]
pub struct NewLap {
    pub heat: i32,
    pub driver: i32,
    pub lap_in_heat: i32,
    pub lap_time: f64,
    pub kart_id: i32,
}

impl NewLap {
    pub fn into_lap(&self, id: i32) -> Lap {
        Lap {
            id,
            heat: self.heat,
            driver:self.driver,
            lap_in_heat: self.lap_in_heat,
            lap_time: self.lap_time,
            kart_id: self.kart_id,
        }
    }
}

#[derive(Serialize, PartialEq, Debug, Clone, Deserialize, HasId)]
pub struct Lap {
    pub id: i32,
    pub heat: i32,
    pub driver: i32,
    pub lap_in_heat: i32,
    pub lap_time: f64,
    pub kart_id: i32,
}

impl Into<i32> for Lap {
    fn into(self) -> i32 {
        self.id
    }
}

impl Into<Lap> for crate::cornucopia::queries::laps::Lap {
    fn into(self) -> Lap {
        Lap {
            id: self.id,
            heat: self.heat,
            driver: self.driver,
            lap_in_heat: self.lap_in_heat,
            lap_time: self.lap_time,
            kart_id: self.kart_id,
        }
    }
}

impl Into<i32> for &Lap {
    fn into(self) -> i32 {
        self.id
    }
}

impl Lap {
    /************ INSERTERS ************/

    /// # insert multiple laps into the database
    /// insert multiple laps into the database from a vector of NewLap objects
    ///
    /// ## Arguments
    /// * `new_laps` - The new laps to insert
    ///
    /// ## Returns
    /// * `Vec<Lap>` - The inserted laps
    pub async fn insert_bulk(new_laps: &Vec<NewLap>) -> Vec<Lap> {
        let mut heats: Vec<i32> = Vec::new();
        let mut drivers: Vec<i32> = Vec::new();
        let mut laps_in_heat: Vec<i32> = Vec::new();
        let mut lap_times: Vec<f64> = Vec::new();
        let mut karts: Vec<i32> = Vec::new();

        for lap in new_laps {
            heats.push(lap.heat);
            drivers.push(lap.driver);
            laps_in_heat.push(lap.lap_in_heat);
            lap_times.push(lap.lap_time);
            karts.push(lap.kart_id);
        }

        let client = &get_pool().get().await.unwrap();
        select_vec_from_db!(
            insert_laps_bulk(),
            client,
            &heats,
            &drivers,
            &laps_in_heat,
            &lap_times,
            &karts)
    }

    /************ GETTERS ************/
    /// # Get a lap by its id
    /// get a lap from the database by its id
    ///
    /// ## Arguments
    /// * `id_in` - The id of the lap to get
    ///
    /// ## Returns
    /// * `Lap` - The lap with the given id
    pub async fn from_id(id: i32) -> Lap {
        let client = &get_pool().get().await.unwrap();

        select_from_db!(get_lap(), client, &id)
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
    pub async fn from_kart(kart: &Vehicle) -> Vec<Lap> {
        let client = &get_pool().get().await.unwrap();
        select_vec_from_db!(get_laps_from_kart(), client, &kart.id)
    }

    pub fn from_kart_offline(all_laps: &[Lap], kart: &Vehicle) -> Vec<Lap> {
        all_laps
            .into_iter()
            .filter(|lap| lap.kart_id.eq(&kart.id))
            .map(|e| e.to_owned())
            .collect()
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
    pub async fn from_driver(driver: &Driver) -> Vec<Lap> {
        let client = &get_pool().get().await.unwrap();
        select_vec_from_db!(get_laps_from_driver(), client, &driver.id)
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
    pub async fn from_drivers(drivers: &[Driver]) -> Vec<Lap> {
        let client = &get_pool().get().await.unwrap();

        select_vec_from_db!(
            get_laps_from_drivers(),
            client,
            &drivers.iter().map(|e| e.id).collect::<Vec<i32>>())
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
    pub async fn from_drivers_as_map(drivers: &[Driver]) -> HashMap<Driver, Vec<Lap>> {
        let laps = Lap::from_drivers(drivers).await;

        let mut heat_lap_map: HashMap<Driver, Vec<Lap>> = HashMap::new();
        for lap in laps {
            let driver_in = drivers
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

        return heat_lap_map;
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
    pub async fn from_heat(heat: &Session) -> Vec<Lap> {
        let client = &get_pool().get().await.unwrap();
        select_vec_from_db!(get_laps_from_heat(), client, &heat.id)
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
    pub async fn from_heats(heats: &[Session]) -> Vec<Lap> {
        let client = &get_pool().get().await.unwrap();

        let ids: Vec<i32> = heats.iter().map(|e| e.id).collect();
        select_vec_from_db!(get_laps_from_heats(), client, &ids)
    }


    pub fn from_heats_as_map_offline(heats: &[Session], laps: &[Lap]) -> HashMap<Session, Vec<Lap>> {
        let mut heat_lap_map: HashMap<Session, Vec<Lap>> = HashMap::new();
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

}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LapsStats {
    pub avg_lap_time: f64,
    pub median_lap_time: f64,
    pub fastest_lap_time: f64,
}