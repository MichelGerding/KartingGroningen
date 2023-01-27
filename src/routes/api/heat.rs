use std::collections::HashMap;

use chrono::NaiveDateTime;
use json_response_derive::JsonResponse;
use log::{error, info, warn};
use rocket::{FromForm, get, post};
use rocket::form::Form;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::http::uri::Origin;
use rocket::Request;
use rocket::response::{Flash, Redirect};
use rocket::response;
use rocket::response::Responder;
use rocket::response::Response;
use rocket::serde::Deserialize;
use serde::Serialize;

use crate::macros::database_error_handeler::db_handle_get_error_http;
use crate::macros::request_caching::{cache_response, read_cache_request};
use crate::modules::heat_api::{get_heats_from_api, save_heat, WebResponse};
use crate::modules::models::driver::{Driver, sanitize_name};
use crate::modules::models::general::establish_connection;
use crate::modules::models::heat::{Heat, HeatStats};
use crate::modules::models::kart::Kart;
use crate::modules::models::lap::Lap;
use crate::modules::redis::Redis;

/**************************************************************************************************/
/**************** ROUTES **************************************************************************/
/**************************************************************************************************/

/***** MODIFY HEATS *****/

/// # load a new heat into the db
#[post("/heats/new", data = "<new_heat>")]
pub async fn save_one(new_heat: Form<NewHeatFormData>) -> Result<Flash<Redirect>, Status> {
    let sanitized = sanitize_name(&new_heat.heat_id);
    if sanitized != new_heat.heat_id {
        return Ok(Flash::error(
            Redirect::to("/"),
            format!("|Invalid heat id: {}", new_heat.heat_id),
        ));
    }


    let conn = &mut establish_connection();

    let heat = new_heat.into_inner().heat_id;
    let response = get_heats_from_api(vec![heat]).await;
    if response.len() == 0 {
        return Ok(Flash::warning(
            Redirect::to(format!("/", )),
            "|Heat not found"));
    }

    let heat: WebResponse = response[0].clone();
    let heat_id = save_heat(conn, heat).unwrap();

    Ok(Flash::success(
        Redirect::to(format!("/heats/{}", heat_id)),
        "Heat saved"))
}

#[post("/heats/delete/<heat_id>")]
pub async fn delete(heat_id: String) -> Result<Flash<Redirect>, Status> {
    let sanitized = sanitize_name(&heat_id);
    if sanitized != heat_id {
        return Err(Status::BadRequest);
    }

    let conn = &mut establish_connection();

    match Heat::get_by_id(conn, &heat_id) {
        Ok(heat) => match heat.delete(conn) {
            Ok(_) => {
                info!("Deleted heat: {}", heat_id);
                Ok(Flash::success(
                    Redirect::to("/"),
                    format!("Deleted heat: {}", heat_id)))
            },
            Err(e) => {
                error!(target:"routes/api/heat:delete", "Tried deleting heat got error: {}", e);
                return Err(Status::InternalServerError);
            }
        },
        Err(diesel::NotFound) => {
            warn!(target:"routes/api/heat:delete", "Tried deleting heat but it didn't exist");
            return Ok(Flash::success(
                Redirect::to("/"),
                format!("Heat {} deleted", &heat_id)));

        }
        Err(e) => {
            error!(target:"routes/api/heat:delete", "Tried deleting heat got error: {}", e);
            return Err(Status::InternalServerError);
        }
    }
}

#[get("/heats/<heat_id>")]
pub fn get_one_stats(heat_id: String, origin: &Origin) -> Result<HeatStats, Status> {
    let sanitized = sanitize_name(&heat_id);
    if sanitized != heat_id {
        return Err(Status::BadRequest);
    }

    read_cache_request!(origin);

    let connection = &mut establish_connection();
    let heat = db_handle_get_error_http!(Heat::get_with_stats(connection, heat_id), "/routes/api/heat:get_one_stats", "heat stats");

    cache_response!(origin, heat);
}

/***** GETTERS *****/
#[get("/heats/<heat_id>/full")]
pub fn get_one(heat_id: String, origin: &Origin) -> Result<ApiHeat, Status> {
    let sanitized = sanitize_name(&heat_id);
    if sanitized != heat_id {
        return Err(Status::BadRequest);
    }

    read_cache_request!(origin);

    let conn = &mut establish_connection();
    let heat = match Heat::get_by_id(conn, &heat_id) {
        Ok(heat) => heat,
        Err(_) => return Err(Status::NotFound),
    };

    let laps = db_handle_get_error_http!(heat.get_laps(conn), "/routes/api/heat:get_one", "laps");
    let karts = db_handle_get_error_http!(Kart::from_laps(conn, &laps), "/routes/api/heat:get_one", "karts");
    let drivers = db_handle_get_error_http!(Driver::from_laps(conn, &laps), "/routes/api/heat:get_one", "drivers");

    cache_response!(origin, ApiHeat::new(&heat, &drivers, &laps, &karts));
}

/// # get all heats
/// get all heats, laps, drivers, and karts from database
///
///
/// WARNING!! DO NOT USE UNLESS NECESAIRY
/// large amounts of data.
/// dumps idless database
#[get("/heats/all/full")]
pub fn get_all(origin: &Origin) -> Result<String, Status> {
    read_cache_request!(origin);

    let conn = &mut establish_connection();

    let all_heats = db_handle_get_error_http!(Heat::get_all(conn), "/routes/api/heat:get_all", "heats");
    let all_laps = db_handle_get_error_http!(Lap::from_heats_as_map(conn, &all_heats), "/routes/api/heat:get_all", "laps from heat as map");

    let all_drivers = db_handle_get_error_http!(Driver::get_all(conn), "/routes/api/heat:get_all", "drivers");
    let all_karts = db_handle_get_error_http!(Kart::get_all(conn), "/routes/api/heat:get_all", "karts");

    let api_heats: Vec<ApiHeat> = ApiHeat::bulk_new(&all_heats, all_laps, all_drivers, all_karts);

    cache_response!(origin, serde_json::to_string(&api_heats).unwrap());
}

/// # get all heats
/// get info about all heats.
#[get("/heats/all")]
pub fn get_all_ids() -> Result<String, Status> {
    let conn = &mut establish_connection();

    let heats = db_handle_get_error_http!(Heat::get_all_with_stats(conn) , "/routes/api/heat:get_all_ids", "heats with stats");
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
    pub fn new(heat: &Heat, drivers: &[Driver], laps: &[Lap], karts: &[Kart]) -> ApiHeat {
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

    pub fn bulk_new(all_heats: &[Heat], all_laps: HashMap<Heat, Vec<Lap>>, all_drivers: Vec<Driver>, all_karts: Vec<Kart>) -> Vec<ApiHeat> {
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
                let karts = Kart::from_laps_offline(&all_karts, laps);

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
