use crate::modules::database::models::driver::Driver;
use chrono::NaiveDateTime;
use rocket::get;
use rocket::http::Status;
use rocket::serde::Serialize;
use std::collections::HashMap;

use crate::modules::database::models::session::Session;
use crate::modules::database::models::vehicle::Vehicle;
use crate::modules::database::models::lap::Lap;

use json_response_derive::JsonResponse;
use log::error;
use rocket::http::ContentType;
use rocket::response;
use rocket::response::Responder;
use rocket::response::Response;
use rocket::Request;

use crate::macros::request_caching::{cache_generation, cache_response, read_cache_request};
use crate::modules::redis::Redis;
use rocket::http::uri::Origin;
use serde::Deserialize;

#[tokio::main]
pub async fn get_one_cache_gen(kart_number: i32) -> Result<String, Status> {
    match Vehicle::get_with_stats(kart_number).await {
        Some(kart) => return Ok(serde_json::to_string(&kart).unwrap()),
        None => return Err(Status::NotFound),
    };
}


#[get("/karts/<kart_number>")]
pub fn get_one(kart_number: i32, origin: &Origin) -> Result<String, Status> {
    cache_generation!(origin, KartStats, ||{
        get_one_cache_gen(kart_number)
    });
}

#[get("/karts/<kart_number>/full")]
pub async fn get_one_full(kart_number: i32, origin: &Origin<'_>) -> Result<ApiKartResult, Status> {
    read_cache_request!(origin);

    let kart: Vehicle = match Vehicle::get_by_number(kart_number).await {
        Some(kart) => kart,
        None => {
            error!(target:"routes/api/kart:get_one_full", "Error getting kart");
            return Err(Status::InternalServerError);
        }
    };

    let all_laps = Lap::from_kart(&kart).await;
    let all_heats = Session::from_laps(&all_laps).await;
    let result = Driver::from_laps(&all_laps).await;

    Ok(ApiKartResult::new(&kart, &all_laps, &result, &all_heats))
    //TODO:: add caching

    // cache_response!(origin, result);
}

#[get("/karts/all?<sort_col>&<sort_dir>")]
pub async fn get_all(
    origin: &Origin<'_>,
    sort_dir: Option<String>,
    sort_col: Option<String>,
) -> Result<String, Status> {
    read_cache_request!(origin);
    let mut sort_col = sort_col.unwrap_or("number".to_string());
    let mut sort_dir = sort_dir.unwrap_or("asc".to_string());

    if sort_col.is_empty() {
        sort_col = "start_time".to_string();
    }
    if sort_dir.is_empty() || (sort_dir != "desc" && sort_dir != "asc") {
        sort_dir = "asc".to_string();
    }

    let all_karts = Vehicle::get_all_with_stats(sort_col, sort_dir).await;

    cache_response!(origin, serde_json::to_string(&all_karts).unwrap());
}

#[derive(Serialize, Deserialize, JsonResponse)]
pub struct ApiKartResult {
    number: i32,
    brand: String,
    model: String,
    horsepower: i32,
    modified: bool,
    heats: Vec<ApiHeatResult>,
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
    laps: Vec<ApiLapResult>,
}

impl ApiKartResult {
    pub fn new(
        kart: &Vehicle,
        all_laps: &[Lap],
        all_drivers: &[Driver],
        all_heats: &[Session],
    ) -> ApiKartResult {
        let lap_heats: HashMap<Session, Vec<Lap>> =
            Lap::from_heats_as_map_offline(all_heats, all_laps);

        ApiKartResult {
            number: kart.number.clone(),
            brand: kart.brand.clone(),
            model: kart.model.clone(),
            horsepower: kart.horsepower.clone(),
            modified: kart.modified.clone(),
            heats: lap_heats
                .iter()
                .map(|(heat, laps)| {
                    let driver = match all_drivers.iter().find(|d| d.id == laps[0].driver) {
                        None => return None,
                        Some(e) => e.to_owned(),
                    };

                    Some(ApiHeatResult {
                        heat_id: heat.heat_id.clone(),
                        start_date: heat.start_date.clone(),
                        driver: ApiDriverResult {
                            name: driver.name.clone(),
                        },
                        laps: laps
                            .iter()
                            .map(|lap| ApiLapResult {
                                lap_time: lap.lap_time,
                                lap_in_heat: lap.lap_in_heat,
                            })
                            .collect(),
                    })
                })
                .filter(|e| e.is_some())
                .map(|e| e.unwrap())
                .collect(),
        }
    }

    pub fn bulk_new(
        all_karts: Vec<Vehicle>,
        all_laps: Vec<Lap>,
        all_drivers: Vec<Driver>,
        all_heats: Vec<Session>,
    ) -> Vec<ApiKartResult> {
        all_karts
            .iter()
            .map(|kart| {
                let kart_laps = Lap::from_kart_offline(&all_laps, kart);
                let kart_heats = Session::from_laps_offline(&all_heats, &kart_laps);
                let kart_drivers = Driver::from_laps_offline(&all_drivers, &kart_laps);

                ApiKartResult::new(kart, &kart_laps, &kart_drivers, &kart_heats)
            })
            .collect()
    }
}
