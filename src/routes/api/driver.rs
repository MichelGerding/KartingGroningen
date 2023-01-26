use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

use std::collections::HashMap;
use chrono::NaiveDateTime;

use rocket::{get, FromForm};
use rocket::http::uri::Origin;
use serde::{Deserialize, Serialize};
use json_response_derive::JsonResponse;

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
    let driver = Driver::get_driver_with_stats(connection, driver_name);

    cache_response!(origin, driver);
}

#[get("/drivers/<driver_name>/full", rank = 1)]
pub fn get_one(driver_name: String, origin: &Origin) -> Result<ApiDriver, Status> {
    // check if the input is valid
    let sanitized = sanitize_name(&driver_name);
    if sanitized != driver_name {
        return Err(Status::BadRequest);
    }

    // check if request is cached.
    // faster to check input then to make a request to the cache
    read_cache_request!(origin);

    let conn = &mut establish_connection();
    let driver = match Driver::get_by_name(conn, &driver_name) {
        Ok(d)  => d,
        Err(_) => return Err(Status::NotFound),
    };

    let laps = driver.get_laps(conn);
    let heats = Heat::from_laps(conn, &laps);
    let karts = Kart::from_laps(conn, &laps);


    let api_driver = ApiDriver::new(&driver, &heats, &laps, &karts);

    cache_response!(origin, api_driver.clone());
}


#[get("/drivers/search/full?<name>&<page>&<page_size>")]
pub fn search_full(name: String, page: Option<i32>, page_size: Option<i32>) -> Result<String, Status> {
    let sanitized = sanitize_name(&name);
    if sanitized != name {
        return Err(Status::BadRequest);
    }

    let conn = &mut establish_connection();

    let drivers;

    if page.is_none() || page_size.is_none() {
        drivers = match Driver::search_by_name(conn, &name) {
            Ok(d)  => d,
            Err(_) => return Err(Status::NotFound),
        };
    } else {
        drivers = match Driver::search_by_name_paginated(conn, &name, page.unwrap(), page_size.unwrap()) {
            Ok(d)  => d,
            Err(_) => return Err(Status::NotFound),
        };
    }


    let all_laps_map = Lap::from_drivers_as_map(conn, &drivers);

    let all_laps: Vec<Lap> = all_laps_map
        .iter()
        .map(|e| e.1)
        .flatten()
        .map(|e| e.to_owned())
        .collect();

    let all_heats = Heat::from_laps(conn, &all_laps);
    let all_karts = Kart::from_laps(conn, &all_laps);

    let api_drivers: Vec<ApiDriver> = ApiDriver::bulk_new(&drivers, &all_laps_map, &all_heats, &all_karts);
    Ok(serde_json::to_string(&api_drivers).unwrap())
}


#[get("/drivers/search?<name>&<page>&<page_size>")]
pub fn search(name: String, page: Option<u32>, page_size: Option<u32>) -> Result<String, Status> {
    let sanitized = sanitize_name(&name);
    if sanitized != name {
        return Err(Status::BadRequest);
    }

    let conn = &mut establish_connection();

    let drivers;
    if page.is_none() || page_size.is_none() {
        drivers = Driver::search_with_stats(conn, name.clone());
    } else {
        drivers = Driver::search_with_stats_paginated(
            conn,
            name.clone(),
            page_size.unwrap(),
            page.unwrap());
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
    let drivers = serde_json::to_string(&Driver::get_all_with_stats(conn)).unwrap();

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

    let all_heats = Heat::get_all(conn);
    let all_drivers = Driver::get_all(conn);
    let all_karts = Kart::get_all(conn);
    let all_laps = Lap::from_drivers_as_map(conn, &all_drivers);

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
