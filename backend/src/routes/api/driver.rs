use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

use std::collections::HashMap;
use chrono::NaiveDateTime;

use rocket::{get, FromForm};
use rocket::http::uri::Origin;
use serde::{Deserialize, Serialize};
use json_response_derive::JsonResponse;
use log::{error};
use crate::macros::database_error_handeler::db_handle_get_error_http;

use crate::modules::models::driver::{Driver, DriverStats, sanitize_name};
use crate::modules::models::general::establish_connection;
use crate::modules::models::heat::Heat;
use crate::modules::models::kart::Kart;
use crate::modules::models::lap::Lap;
use crate::routes::api::heat::ApiLap;
use crate::modules::redis::Redis;

use crate::macros::request_caching::{read_cache_request, cache_response};

/**************************************************************************************************/
/**************** ROUTES **************************************************************************/
/**************************************************************************************************/

pub struct Paginated {
    pub limit: u32,
    pub page: u32,
}


#[get("/drivers/<driver_name>", rank = 1)]
pub fn get_one_stats(driver_name: String, origin: &Origin) -> Result<DriverStats, Status> {
    let sanitized = sanitize_name(&driver_name);
    if sanitized != driver_name {
        return Err(Status::BadRequest);
    }

    read_cache_request!(origin);

    let connection = &mut establish_connection();
    let driver = match Driver::get_driver_with_stats(connection, driver_name) {
        Ok(driver) => driver,
        Err(diesel::result::Error::NotFound) => return Err(Status::NotFound),
        Err(error) => {
            error!(target:"routes/api/driver:get_one_stats", "Error getting driver: {}", error);
            return Err(Status::InternalServerError);
        }
    };

    cache_response!(origin, driver);
}

#[get("/drivers/<driver_name>/full", rank = 1)]
pub fn get_one(driver_name: String, origin: &Origin) -> Result<ApiDriver, Status> {
    // check if the input is valid
    let sanitized = sanitize_name(&driver_name);
    if sanitized != driver_name {
        println!("{} != {}", sanitized, driver_name);
        return Err(Status::BadRequest);
    }

    // check if request is cached.
    // faster to check input then to make a request to the cache
    read_cache_request!(origin);

    let conn = &mut establish_connection();
    let driver = db_handle_get_error_http!(Driver::get_by_name(conn, &driver_name), "routes/api/driver:get_one", "driver");

    let laps = db_handle_get_error_http!(driver.get_laps(conn), "routes/api/driver:get_one", "laps");
    let heats = db_handle_get_error_http!(Heat::from_laps(conn, &laps), "routes/api/driver:get_one", "heats");
    let karts = db_handle_get_error_http!(Kart::from_laps(conn, &laps), "routes/api/driver:get_one", "karts");

    let api_driver = ApiDriver::new(&driver, &heats, &laps, &karts);

    cache_response!(origin, api_driver.clone());
}


#[get("/drivers/search/full?<q>&<page>&<page_size>")]
pub fn search_full(q: String, page: Option<i32>, page_size: Option<i32>) -> Result<String, Status> {
    let sanitized = sanitize_name(&q);
    if sanitized != q {
        return Err(Status::BadRequest);
    }

    let conn = &mut establish_connection();

    let drivers;


    if page.is_none() || page_size.is_none() {
        drivers = db_handle_get_error_http!(Driver::search_by_name(conn, &q), "routes/api/driver:search_full", "drivers");
    } else {
        drivers = db_handle_get_error_http!(Driver::search_by_name_paginated(conn, &q, page.unwrap(), page_size.unwrap()), "routes/api/driver:search_full", "drivers");
    }


    let all_laps_map = db_handle_get_error_http!(Lap::from_drivers_as_map(conn, &drivers), "routes/api/driver:search_full", format!("laps as map for drivers `%{}%`", q));

    let all_laps: Vec<Lap> = all_laps_map
        .iter()
        .map(|e| e.1)
        .flatten()
        .map(|e| e.to_owned())
        .collect();

    let all_heats = db_handle_get_error_http!(Heat::from_laps(conn, &all_laps), "routes/api/driver:search_full", "heats");
    let all_karts = db_handle_get_error_http!(Kart::from_laps(conn, &all_laps), "routes/api/driver:search_full", "karts");

    let api_drivers: Vec<ApiDriver> = ApiDriver::bulk_new(&drivers, &all_laps_map, &all_heats, &all_karts);
    Ok(serde_json::to_string(&api_drivers).unwrap())
}


#[get("/drivers/search?<q>&<page>&<page_size>&<sort_col>&<sort_dir>")]
pub fn search(q: String, page: Option<u32>, page_size: Option<u32>, sort_col: Option<String>, sort_dir: Option<String>) -> Result<String, Status> {
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

    let conn = &mut establish_connection();

    let drivers;
    if page.is_none() || page_size.is_none() {
        drivers = Driver::search_with_stats(conn, q.clone(), sort_col, sort_dir);
    } else {
        drivers = Driver::search_with_stats_paginated(
            conn,
            q.clone(),
            page_size.unwrap(),
            page.unwrap(),
            sort_col,
            sort_dir,);
    }

    let drivers = match drivers {
        Ok(drivers) => drivers,
        Err(_) => {
            return Err(Status::NotFound);
        }
    };

    Ok(serde_json::to_string(&drivers).unwrap())
}


#[get("/drivers/all")]
pub fn get_all_ids(origin: &Origin) -> Result<String, Status> {
    read_cache_request!(origin);

    let conn = &mut establish_connection();
    let drivers = match Driver::get_all_with_stats(conn) {
        Ok(drivers) => serde_json::to_string(&drivers).unwrap(),
        Err(diesel::result::Error::NotFound) => return Err(Status::NotFound),
        Err(_) => return Err(Status::InternalServerError),
    };

    cache_response!(origin, drivers);
}


/// # get all drivers
/// get all heats, laps, drivers, and karts from database
///
///
/// WARNING!! DO NOT USE UNLESS NECESAIRY
/// large amounts of data.
/// dumps id less database
#[get("/drivers/all/full")]
pub fn get_all(origin: &Origin) -> Result<String, Status> {
    read_cache_request!(origin);


    let conn = &mut establish_connection();

    let all_heats = db_handle_get_error_http!(Heat::get_all(conn), "routes/api/driver:get_all", "heats");
    let all_drivers = db_handle_get_error_http!(Driver::get_all(conn), "routes/api/driver:get_all", "heats");
    let all_karts = db_handle_get_error_http!(Kart::get_all(conn), "routes/api/driver:get_all", "heats");
    let all_laps = db_handle_get_error_http!(Lap::from_drivers_as_map(conn, &all_drivers), "routes/api/driver:get_all", "laps from drivers as map");

    let api_drivers: Vec<ApiDriver> = ApiDriver::bulk_new(&all_drivers, &all_laps, &all_heats, &all_karts);

    cache_response!(origin, serde_json::to_string(&api_drivers).unwrap());
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
    pub fn new(driver: &Driver, heats: &[Heat], laps: &[Lap], karts: &[Kart]) -> ApiDriver {
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
                            number: kart.number,
                            is_child_kart: kart.is_child_kart,
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

    pub fn bulk_new(drivers: &[Driver], all_laps: &HashMap<Driver, Vec<Lap>>, all_heats: &[Heat], all_karts: &[Kart]) -> Vec<ApiDriver>{
        drivers
            .iter()
            .map(|driver| {
                let laps = all_laps.get(driver).unwrap();
                let heats = Heat::from_laps_offline(&all_heats, laps);
                let karts = Kart::from_laps_offline(&all_karts, laps);

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
    pub is_child_kart: bool,
}