use std::collections::HashMap;
use crate::models::{NewLap};
use crate::modules::helpers::math::Math;
use crate::{TemplateDataDriver, TemplateDataLap};

pub struct LapHelper {}

pub struct LapInfo {
    pub driver_name: String,
    pub lap_in_heat: i32,
}

impl LapHelper {
    pub fn get_standard_deviation_of_laps(lap_times: &[TemplateDataLap]) -> f64 {
        let laptimes: Vec<f64> = lap_times.iter().map(|lap| lap.lap_time).collect();
        Math::standard_deviation(&laptimes)
    }

    pub fn get_center_of_laps(lap_times: &Vec<TemplateDataLap>) -> f64 {
        let mut lap_times_sorted = lap_times.clone();
        lap_times_sorted.sort_by(|a, b| a.lap_time.partial_cmp(&b.lap_time).unwrap());
        let middle = lap_times_sorted.len() / 2;
        if lap_times_sorted.len() % 2 == 0 {
            (lap_times_sorted[middle - 1].lap_time + lap_times_sorted[middle].lap_time) / 2.0
        } else {
            lap_times_sorted[middle].lap_time
        }
    }



    pub fn get_laps_at_time(drivers: Vec<TemplateDataDriver>, seconds: f64) -> HashMap<String, NewLap> {
        let mut laps: HashMap<String, NewLap> = HashMap::new();

        for driver in drivers {

            let mut start = 0.0;
            for lap in driver.all_laps {

                let end = start + lap.lap_time;
                if start <= seconds  && seconds <= end {
                    laps.insert( driver.driver_name.clone(), NewLap{
                        lap_time: lap.lap_time,
                        lap_in_heat: lap.lap_in_heat,
                        driver: -1,
                        heat: -1,
                        kart_id: -1,
                    });

                    break;
                }
                start += lap.lap_time;
            }
        }

        laps
    }

    pub fn get_outlier_laps(laps: &Vec<TemplateDataLap>) -> Vec<TemplateDataLap> {
        // we expect all drivers to be the same so we only look at the lapstimes
        let mut outliers: Vec<TemplateDataLap> = Vec::new();

        // get the standard deviation of the laptimes in vec laps
        let mut lap_times: Vec<f64> = Vec::new();
        for lap in laps.iter() {
            lap_times.push(lap.lap_time);
        }
        let standard_deviation = LapHelper::get_standard_deviation_of_laps(laps);

        // get the center of the laptimes
        let center = LapHelper::get_center_of_laps(laps);

        // get the outliers
        for lap in laps.iter() {
            if lap.lap_time > center + (standard_deviation * 2.0) || lap.lap_time < 45.0 {
                outliers.push(lap.clone());
            }
        }

        outliers
    }
}


