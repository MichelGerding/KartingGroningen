use diesel::PgConnection;
use crate::modules::models::kart::Kart;
use crate::modules::models::lap::Lap;
use crate::{TemplateDataDriver, TemplateDataLap};

use crate::modules::helpers::general::Helpers;
use crate::modules::helpers::lap::LapHelper;
use crate::modules::helpers::math::Math;

use crate::modules::models::driver::Driver;

pub struct DriverHelpers{}


impl DriverHelpers {
    pub fn get_stats_for_laps(conn: &mut PgConnection, driver: &Driver, laps: &Vec<Lap>) -> TemplateDataDriver {
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
            if lap.driver == driver.id {
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

        // get the kart the laps are driven in. we expect all laps to be driven in the same kart
        let kart = Kart::get_by_id(conn, _lap_of_driver.kart_id);

        // separate the normal and abnormal laps
        let outliers: Vec<TemplateDataLap> = LapHelper::get_outlier_laps(&laps_of_driver);
        let normal_laps: Vec<TemplateDataLap> = Helpers::get_difference_between_vectors(&laps_of_driver, &outliers);


        TemplateDataDriver {
            driver_name: driver.name.clone(),
            fastest_lap,
            all_laps: laps_of_driver.to_vec(),
            normal_laps: normal_laps.to_vec(),
            outlier_laps: outliers.to_vec(),
            total_laps: laps_of_driver.len(),
            avg_lap_time: Math::round_float_to_n_decimals(total_lap_time / laps_of_driver.len() as f64, 3),
            kart: kart.number,
        }
    }
}
