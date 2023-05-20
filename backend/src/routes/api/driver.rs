use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

use chrono::NaiveDateTime;
use std::collections::HashMap;

use json_response_derive::JsonResponse;
use log::error;
use rocket::http::uri::Origin;
use rocket::{get, FromForm};
use serde::{Deserialize, Serialize};

use crate::modules::database::models::driver::{sanitize_name, Driver, DriverStats};
use crate::modules::database::models::session::Session;
use crate::modules::database::models::vehicle::Vehicle;
use crate::modules::database::models::lap::Lap;
use crate::modules::redis::Redis;
use crate::routes::api::heat::ApiLap;

use crate::macros::request_caching::{cache_response, read_cache_request};
/**************************************************************************************************/
/**************** ROUTES **************************************************************************/
/**************************************************************************************************/

pub struct Paginated {
    pub limit: u32,
    pub page: u32,
}

#[get("/drivers/<driver_name>", rank = 1)]
pub async fn get_one_stats(driver_name: String, origin: &Origin<'_> ) -> Result<DriverStats, Status> {
    let sanitized = sanitize_name(&driver_name);
    if sanitized != driver_name {
        return Err(Status::BadRequest);
    }

    read_cache_request!(origin);

    let driver = Driver::get_driver_with_stats(driver_name).await;

    cache_response!(origin, driver);
}

#[get("/drivers/<driver_name>/full", rank = 1)]
pub async fn get_one(driver_name: String, origin: &Origin<'_>) -> Result<ApiDriver, Status> {
    // check if the input is valid
    let sanitized = sanitize_name(&driver_name);
    if sanitized != driver_name {
        println!("{} != {}", sanitized, driver_name);
        return Err(Status::BadRequest);
    }

    // check if request is cached.
    // faster to check input then to make a request to the cache
    read_cache_request!(origin);

    let driver = Driver::get_by_name(&driver_name).await;
    let laps = driver.get_laps().await;
    let heats = Session::from_laps(&laps).await;
    let karts = Vehicle::from_laps(&laps).await;

    let api_driver = ApiDriver::new(&driver, &heats, &laps, &karts);
    cache_response!(origin, api_driver.clone());
}

#[get("/drivers/search/full?<q>&<page>&<page_size>")]
pub async fn search_full(q: String, page: Option<i32>, page_size: Option<i32>) -> Result<String, Status> {
    let sanitized = sanitize_name(&q);
    if sanitized != q {
        return Err(Status::BadRequest);
    }

    let drivers = Driver::search_by_name(
        &q,
        page.unwrap_or(0),
        page_size.unwrap_or(500)).await;

    let all_laps_map = Lap::from_drivers_as_map(&drivers).await;

    let all_laps: Vec<Lap> = all_laps_map
        .iter()
        .map(|e| e.1)
        .flatten()
        .map(|e| e.to_owned())
        .collect();

    let all_heats = Session::from_laps(&all_laps).await;
    let all_karts = Vehicle::from_laps(&all_laps).await;

    let api_drivers: Vec<ApiDriver> =
        ApiDriver::bulk_new(&drivers, &all_laps_map, &all_heats, &all_karts);
    Ok(serde_json::to_string(&api_drivers).unwrap())
}

#[get("/drivers/search?<q>&<page>&<page_size>&<sort_col>&<sort_dir>")]
pub async fn search(
    q: String,
    page: Option<u32>,
    page_size: Option<u32>,
    sort_col: Option<String>,
    sort_dir: Option<String>,
) -> Result<String, Status> {
    let sanitized = sanitize_name(&q);
    if sanitized != q {
        return Err(Status::BadRequest);
    }

    let mut sort_col = sort_col.unwrap_or("name".to_string());
    let mut sort_dir = sort_dir.unwrap_or("asc".to_string());

    if sort_col.is_empty() {
        sort_col = "name".to_string();
    }
    if sort_dir.is_empty() || (sort_dir != "desc" && sort_dir != "asc") {
        sort_dir = "asc".to_string();
    }


    let drivers = Driver::search_with_stats(
        q.clone(),
        page_size.unwrap_or(10),
        page.unwrap_or(1),
        sort_col,
        sort_dir,
    ).await;

    let drivers = drivers;

    Ok(serde_json::to_string(&drivers).unwrap())
}

#[get("/drivers/all")]
pub async fn get_all_ids(origin: &Origin<'_>) -> Result<String, Status> {
    read_cache_request!(origin);

    let drivers = Driver::get_all_with_stats().await;
    Ok(serde_json::to_string(&drivers).unwrap())
    // cache_response!(origin, drivers);
}

/**************************************************************************************************/
/**************** HELPERS *************************************************************************/
/**************************************************************************************************/

#[derive(FromForm)]
pub struct NewHeatFormData {
    pub heat_id: String,
}

/// # Struct representing a json response for a heat
#[derive(Serialize, Deserialize, Clone, JsonResponse)]
pub struct ApiDriver {
    pub name: String,
    pub rating: f64,
    pub heats: Vec<ApiHeat>,
}

impl ApiDriver {
    /// # Create a object to represent the heat and its driven laps.
    /// we expect that the drivers and laps are for the given heat.
    /// We also expect that a driver has only driven in a single kart.
    ///
    /// # Arguments
    /// * `heat` - The heat to represent
    /// * `laps` - The laps driven in the heat
    /// * `drivers` - The drivers that drove in the heat
    /// * `karts` - The karts that were driven in the heat
    pub fn new(driver: &Driver, heats: &[Session], laps: &[Lap], karts: &[Vehicle]) -> ApiDriver {
        ApiDriver {
            name: driver.name.to_string(),
            rating: driver.rating,
            heats: heats
                .iter()
                .map(|heat| {
                    let driver_laps = laps
                        .iter()
                        .filter(|lap| lap.heat == heat.id)
                        .collect::<Vec<&Lap>>();
                    let kart_id = driver_laps.first().unwrap().kart_id;
                    let kart = karts.iter().find(|kart| kart.id == kart_id).unwrap();

                    ApiHeat {
                        heat_id: heat.heat_id.to_string(),
                        start_date: heat.start_date,
                        kart: ApiKart {
                            number: kart.number.clone(),
                            brand: kart.brand.clone(),
                            model: kart.model.clone(),
                            horsepower: kart.horsepower.clone(),
                            modified: kart.modified.clone(),
                        },
                        laps: laps
                            .iter()
                            .filter(|l| l.heat.eq(&heat.id))
                            .map(|lap| ApiLap {
                                lap_number: lap.lap_in_heat,
                                lap_time: lap.lap_time,
                            })
                            .collect(),
                    }
                })
                .collect(),
        }
    }

    pub fn bulk_new(
        drivers: &[Driver],
        all_laps: &HashMap<Driver, Vec<Lap>>,
        all_heats: &[Session],
        all_karts: &[Vehicle],
    ) -> Vec<ApiDriver> {
        drivers
            .iter()
            .map(|driver| {
                let laps = all_laps.get(driver).unwrap();
                let heats = Session::from_laps_offline(&all_heats, laps);
                let karts = Vehicle::from_laps_offline(&all_karts, laps);

                ApiDriver::new(driver, &heats, laps, &karts)
            })
            .collect()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiHeat {
    pub heat_id: String,
    pub start_date: NaiveDateTime,
    pub kart: ApiKart,
    pub laps: Vec<ApiLap>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiKart {
    pub number: i32,
    pub brand: String,
    pub model: String,
    pub horsepower: i32,
    pub modified: bool,
}
