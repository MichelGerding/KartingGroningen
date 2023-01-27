use std::collections::HashMap;
use chrono::NaiveDateTime;
use rocket::{get};
use rocket::http::Status;
use rocket::serde::Serialize;
use crate::modules::models::driver::Driver;

use crate::modules::models::general::establish_connection;
use crate::modules::models::heat::Heat;
use crate::modules::models::kart::{Kart, KartStats};
use crate::modules::models::lap::Lap;

use rocket::response;
use rocket::Request;
use rocket::response::Responder;
use rocket::response::Response;
use rocket::http::ContentType;
use json_response_derive::JsonResponse;
use log::error;

use crate::modules::redis::Redis;
use crate::macros::request_caching::{cache_response, read_cache_request};
use rocket::http::uri::Origin;
use serde::Deserialize;
use crate::macros::database_error_handeler::db_handle_get_error_http;


#[get("/karts/<kart_number>")]
pub fn get_one(kart_number: i32, origin: &Origin) -> Result<KartStats, Status> {
    read_cache_request!(origin);

    let connection = &mut establish_connection();
    let kart = match Kart::get_with_stats(connection, kart_number) {
        Ok(kart) => kart,
        Err(_) => return Err(Status::NotFound),
    };

    cache_response!(origin, kart);
}

#[get("/karts/<kart_number>/full")]
pub fn get_one_full(kart_number: i32, origin: &Origin) -> Result<ApiKartResult, Status> {
    read_cache_request!(origin);

    let connection = &mut establish_connection();
    let kart: Kart = match Kart::get_by_number(connection, kart_number) {
        Ok(kart) => kart,
        Err(diesel::result::Error::NotFound) => return Err(Status::NotFound),
        Err(e) => {
            error!(target:"routes/api/kart:get_one_full", "Error getting kart: {}", e);
            return Err(Status::InternalServerError);
        },
    };

    let all_laps = db_handle_get_error_http!(Lap::from_kart(connection, &kart), "/routes/api/kart:get_one_full", format!("laps for kart: {}", &kart.number));
    let all_heats = db_handle_get_error_http!(Heat::from_laps(connection, &all_laps), "/routes/api/kart:get_one_full", format!("heats for kart: {}", &kart.number));

    let result = match Driver::from_laps(connection, &all_laps) {
        Ok(all_drivers) => ApiKartResult::new(&kart, &all_laps, &all_drivers, &all_heats),
        Err(diesel::result::Error::NotFound) => return Err(Status::NotFound),
        Err(error) => {
            error!(target:"routes/api/kart:get_one_full", "Error getting driver: {}", error);
            return Err(Status::InternalServerError);
        }
    };

    cache_response!(origin, result);

}

#[get("/karts/all")]
pub fn get_all(origin: &Origin) -> Result<String, Status> {
    read_cache_request!(origin);

    let conn = &mut establish_connection();
    let all_karts = Kart::get_all_with_stats(conn);

   cache_response!(origin, serde_json::to_string(&all_karts).unwrap());
}


/// # get all info about all karts
///
/// !!!! DO NOT USE !!!!
/// loads all data from database so returns giant amounts of data
#[get("/karts/all/full")]
pub fn get_all_full(origin: &Origin) -> Result<String, Status> {
    read_cache_request!(origin);

    let connection = &mut establish_connection();

    // get the kart and stats
    let all_karts = db_handle_get_error_http!(Kart::get_all(connection), "/routes/api/kart:get_all_full", "all karts");
    let all_laps = db_handle_get_error_http!(Lap::get_all(connection), "/routes/api/kart:get_all_full", "all karts");
    let all_drivers = db_handle_get_error_http!(Driver::get_all(connection), "/routes/api/kart:get_all_full", "all karts");
    let all_heats = db_handle_get_error_http!(Heat::get_all(connection), "/routes/api/kart:get_all_full", "all karts");

    let api_karts: Vec<ApiKartResult> = ApiKartResult::bulk_new(all_karts, all_laps, all_drivers, all_heats);

    cache_response!(origin, serde_json::to_string(&api_karts).unwrap());
}


#[derive(Serialize, Deserialize, JsonResponse)]
pub struct ApiKartResult {
    number: i32,
    is_child_kart: bool,
    heats: Vec<ApiHeatResult>
}

#[derive(Serialize, Deserialize)]
pub struct ApiDriverResult {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiLapResult {
    lap_in_heat: i32,
    lap_time: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ApiHeatResult {
    heat_id: String,
    start_date: NaiveDateTime,
    driver: ApiDriverResult,
    laps: Vec<ApiLapResult>
}

impl ApiKartResult {
    pub fn new(kart: &Kart, all_laps: &[Lap], all_drivers: &[Driver], all_heats: &[Heat]) -> ApiKartResult {
        let lap_heats: HashMap<Heat, Vec<Lap>> = Lap::from_heats_as_map_offline(all_heats, all_laps);

        ApiKartResult {
            number: kart.number,
            is_child_kart: kart.is_child_kart,
            heats: lap_heats.iter().map(|(heat, laps)| {
                let driver = match all_drivers.iter().find(|d| d.id == laps[0].driver) {
                    None => return None,
                    Some(e) => e.to_owned()
                };

                Some(ApiHeatResult {
                    heat_id: heat.heat_id.clone(),
                    start_date: heat.start_date.clone(),
                    driver: ApiDriverResult {
                        name: driver.name.clone()
                    },
                    laps: laps.iter().map(|lap| ApiLapResult {
                        lap_time: lap.lap_time,
                        lap_in_heat: lap.lap_in_heat
                    }).collect(),
                })
            }).filter(|e| e.is_some()).map(|e| e.unwrap()).collect(),
        }
    }

    pub fn bulk_new(all_karts: Vec<Kart>, all_laps: Vec<Lap>, all_drivers: Vec<Driver>, all_heats: Vec<Heat>) -> Vec<ApiKartResult> {
        all_karts.iter().map(|kart| {
            let kart_laps = Lap::from_kart_offline(&all_laps, kart);
            let kart_heats = Heat::from_laps_offline(&all_heats, &kart_laps);
            let kart_drivers = Driver::from_laps_offline(&all_drivers, &kart_laps);

            ApiKartResult::new(kart, &kart_laps, &kart_drivers, &kart_heats)
        }).collect()
    }
}