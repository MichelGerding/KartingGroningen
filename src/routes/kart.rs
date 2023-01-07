use std::collections::HashMap;
use chrono::NaiveDate;
use rocket::{get};
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};

use crate::modules::models::general::establish_connection;
use crate::modules::models::lap::{Lap, LapDriver};

use crate::modules::models::driver::Driver;
use crate::modules::models::kart::Kart;
use crate::modules::helpers::driver::DriverHelpers;
use crate::TemplateDataDriver;


#[get("/all")]
pub fn list_all() -> Template {
    let connection = &mut establish_connection();
    let results: Vec<Kart> = Kart::get_all(connection);


    #[derive(Serialize, Deserialize, Clone)]
    struct TemplateDataKart {
        number: i32,
        is_child_kart: bool,
        total_laps: i32,
        total_drivers: i32,
        bg: String,
    }
    let mut results_vec: Vec<TemplateDataKart> = Vec::new();
    for result in &results {
        let laps_of_kart = Lap::from_kart(connection, result);
        let drivers_of_kart = Driver::from_laps(connection, &laps_of_kart);

        let mut bg = "bg-white";
        if result.is_child_kart.unwrap_or(false) {
            bg = "bg-gray-200";
        }

        results_vec.push(TemplateDataKart {
            number: result.number,
            is_child_kart: result.is_child_kart.unwrap_or(false),
            total_laps: laps_of_kart.len() as i32,
            total_drivers: drivers_of_kart.len() as i32,
            bg: bg.to_string(),
        })
    }
    // get the total number of laps each kart has driven


    Template::render("all_karts", context! {
        results: &results_vec,
    })

}

#[get("/single/<kart_number>")]
pub fn single(kart_number: i32) -> Template {
    let connection = &mut establish_connection();
    let kart: Kart = Kart::get_by_number(connection, kart_number);

    let laps = Lap::from_kart(connection, &kart);
    let drivers = Driver::from_laps(connection, &laps);


    let mut template_data = TemplateDataKart {
        number: kart.number,
        is_child_kart: kart.is_child_kart.unwrap_or(false),
        total_laps: laps.len() as i32,
        total_drivers: drivers.len() as i32,
        drivers: Vec::new(),
        lap_drivers: kart.get_laps_driver_and_heat(connection),
        laps_avg_per_day: kart.get_laps_avg_per_day(connection),
        laps_min_per_day: kart.get_minimum_laptime_per_day(connection),
        laps_median_per_day: kart.get_median_laptime_per_day(connection),
    };

    // get stats for each driver
    for driver in &drivers {
        template_data.drivers.push(DriverHelpers::get_stats_for_laps(connection, driver, &laps));
    }

    Template::render("kart", context! {
        data: &template_data,
        json_data: serde_json::to_string(&template_data).unwrap(),
    })
}

#[derive(Serialize, Deserialize, Clone)]
struct TemplateDataKart {
    pub number: i32,
    pub is_child_kart: bool,
    pub total_laps: i32,
    pub total_drivers: i32,

    pub drivers: Vec<TemplateDataDriver>,
    pub lap_drivers: Vec<LapDriver>,
    pub laps_avg_per_day: HashMap<NaiveDate, f64>,
    pub laps_min_per_day: HashMap<NaiveDate, f64>,
    pub laps_median_per_day: HashMap<NaiveDate, f64>,
}