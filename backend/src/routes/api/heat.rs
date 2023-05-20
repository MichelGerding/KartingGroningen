use std::collections::HashMap;

use chrono::NaiveDateTime;
use json_response_derive::JsonResponse;
use log::error;
use rocket::form::Form;
use rocket::http::uri::Origin;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response;
use rocket::response::Responder;
use rocket::response::Response;
use rocket::serde::Deserialize;
use rocket::Request;
use rocket::{get, post, FromForm};
use serde::Serialize;

use crate::macros::request_caching::{cache_response, read_cache_request};
use crate::modules::heat_api::{get_heats_from_api, save_heat, WebResponse};
use crate::modules::database::models::driver::{sanitize_name, Driver};
use crate::modules::database::models::session::{Session, HeatStats};
use crate::modules::database::models::vehicle::Vehicle;
use crate::modules::database::models::lap::Lap;
use crate::modules::redis::Redis;

/**************************************************************************************************/
/**************** ROUTES **************************************************************************/
/**************************************************************************************************/

/***** MODIFY HEATS *****/

/// # load a new heat into the db
#[post("/heats/new", data = "<new_heat>")]
pub async fn save_one(new_heat: Form<NewHeatFormData>) -> Status {
    let sanitized = sanitize_name(&new_heat.heat_id);
    if sanitized != new_heat.heat_id {
        return Status::BadRequest;
    }


    let heat = new_heat.into_inner().heat_id;
    let response = get_heats_from_api(vec![heat]).await;
    if response.len() == 0 {
        return Status::NotFound;
    }

    let heat: WebResponse = response[0].clone();
    save_heat(heat).await.unwrap();

    Status::Ok
}

#[get("/heats/<heat_id>", rank = 1)]
pub async fn get_one_stats(heat_id: String, origin: &Origin<'_>) -> Result<HeatStats, Status> {
    read_cache_request!(origin);

    let heat = Session::get_with_stats(heat_id).await;

    cache_response!(origin, heat);
}

/***** GETTERS *****/
#[get("/heats/<heat_id>/full", rank = 1)]
pub async fn get_one(heat_id: String, origin: &Origin<'_>) -> Result<ApiHeat, Status> {
    read_cache_request!(origin);

    let heat = Session::get_by_id( &heat_id).await;
    let laps = heat.get_laps().await;
    let karts = Vehicle::from_laps(&laps).await;
    let drivers = Driver::from_laps(&laps).await;

    cache_response!(origin, ApiHeat::new(&heat, &drivers, &laps, &karts));
}

/****** SEARSH ROUTES ******/
#[get("/heats/search?<page>&<page_size>&<sort_col>&<sort_dir>")]
pub async fn search(
    page: Option<i64>,
    page_size: Option<i64>,
    sort_dir: Option<String>,
    sort_col: Option<String>,
    origin: &Origin<'_>,
) -> Result<String, Status> {
    let mut sort_col = sort_col.unwrap_or("start".to_string());
    let mut sort_dir = sort_dir.unwrap_or("asc".to_string());

    if sort_col.is_empty() {
        sort_col = "start_time".to_string();
    }
    if sort_dir.is_empty() || (sort_dir != "desc" && sort_dir != "asc") {
        sort_dir = "asc".to_string();
    }

    read_cache_request!(origin);
    let search_results = Session::get_all_paginated(
        page.unwrap_or(0),
        page_size.unwrap_or(500),
        sort_dir,
        sort_col).await;
    cache_response!(origin, serde_json::to_string(&search_results).unwrap());
}

/// # get all heats
/// get info about all heats.
#[get("/heats/all")]
pub async fn get_all_ids() -> Result<String, Status> {
    let heats = Session::get_all_with_stats().await;
    Ok(serde_json::to_string(&heats).unwrap())
}

/**************************************************************************************************/
/**************** HELPERS *************************************************************************/
/**************************************************************************************************/

#[derive(FromForm)]
pub struct NewHeatFormData {
    pub heat_id: String,
}

/// # Struct representing a json response for a heat
#[derive(Serialize, Deserialize, JsonResponse)]
pub struct ApiHeat {
    pub heat_id: String,
    pub heat_type: String,
    pub start_time: NaiveDateTime,
    pub results: Vec<ApiDriverResult>,
}

impl ApiHeat {
    /// # Create a object to represent the heat and its driven laps.
    /// we expect that the drivers and laps are for the given heat.
    /// We also expect that a driver has only driven in a single kart.
    ///
    /// # Arguments
    /// * `heat` - The heat to represent
    /// * `laps` - The laps driven in the heat
    /// * `drivers` - The drivers that drove in the heat
    /// * `karts` - The karts that were driven in the heat
    pub fn new(heat: &Session, drivers: &[Driver], laps: &[Lap], karts: &[Vehicle]) -> ApiHeat {
        ApiHeat {
            heat_id: heat.heat_id.clone(),
            heat_type: heat.heat_type.to_string(),
            start_time: heat.start_date,

            results: drivers
                .iter()
                .map(|driver| {
                    let driver_laps = laps
                        .iter()
                        .filter(|lap| lap.driver == driver.id)
                        .collect::<Vec<&Lap>>();
                    let kart_id = driver_laps.first().unwrap().kart_id;
                    let kart = karts.iter().find(|kart| kart.id == kart_id).unwrap();

                    ApiDriverResult {
                        kart: kart.number,
                        driver: ApiDriver {
                            driver_name: driver.name.to_string(),
                        },
                        laps: driver_laps
                            .iter()
                            .map(|lap| ApiLap {
                                lap_time: lap.lap_time,
                                lap_number: lap.lap_in_heat,
                            })
                            .collect(),
                    }
                })
                .collect(),
        }
    }

    pub fn bulk_new(
        all_heats: &[Session],
        all_laps: HashMap<Session, Vec<Lap>>,
        all_drivers: Vec<Driver>,
        all_karts: Vec<Vehicle>,
    ) -> Vec<ApiHeat> {
        all_heats
            .iter()
            .map(|heat| {
                let laps = all_laps.get(&heat);
                if laps.is_none() {
                    return ApiHeat {
                        heat_id: "".to_string(),
                        heat_type: "".to_string(),
                        start_time: Default::default(),
                        results: vec![],
                    };
                }
                let laps = laps.unwrap();

                let drivers_laps = Driver::map_to_laps(all_drivers.clone(), laps);
                let drivers: Vec<Driver> = drivers_laps.iter().map(|(a, _)| a.to_owned()).collect();
                let karts = Vehicle::from_laps_offline(&all_karts, laps);

                ApiHeat::new(&heat, &drivers, laps, &karts)
            })
            .filter(|e| !e.heat_id.is_empty())
            .collect()
    }
}

/// # Struct representing a json response for a drivers result in a heat
#[derive(Serialize, Deserialize)]
pub struct ApiDriverResult {
    pub kart: i32,
    pub driver: ApiDriver,
    pub laps: Vec<ApiLap>,
}

/// # Struct representing a json response for a Driver
#[derive(Serialize, Deserialize)]
pub struct ApiDriver {
    pub driver_name: String,
}

/// # Struct representing a json response for a Lap
#[derive(Serialize, Deserialize, Clone)]
pub struct ApiLap {
    pub lap_number: i32,
    pub lap_time: f64,
}
