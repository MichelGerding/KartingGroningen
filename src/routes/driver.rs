use std::thread;
// rocket imports
use rocket::get;
use rocket::http::Status;
use rocket_dyn_templates::Template;
// database imports
use crate::modules::models::driver::{Driver, sanitize_name};
use crate::modules::models::general::establish_connection;
use crate::modules::models::heat::Heat;
use crate::modules::models::kart::Kart;
use crate::modules::models::lap::{Lap, LapsStats};
//helper imports
use crate::modules::helpers::math::Math;
use crate::{AllData, ChartData, ChartDataDataSetData, ChartDataDataset, TableData};
use serde::Serialize;


use crate::modules::redis::Redis;
use rocket::http::uri::Origin;
use rocket::serde::Deserialize;
use crate::macros::database_error_handeler::db_handle_get_error_http;
use log::{error};
use crate::macros::redis::{cache_data_to_url, redis_handle_set_error_no_return};


#[get("/all")]
pub fn list_all(origin: &Origin) -> Result<Template, Status> {


    let r_conn_m = &mut Redis::connect();
    if r_conn_m.is_err() {
        error!(target:"routes/driver:list_all", "Error connecting to redis");
        return Err(Status::InternalServerError);
    }
    let r_conn = &mut r_conn_m.as_mut().unwrap();


    let all_data;
    let uri = origin.path().to_string();

    let has_data = match Redis::has_data(r_conn, uri.clone()) {
        Ok(b) => b,
        Err(error) => {
            error!(target:"routes/driver:list_all", "Error checking redis for data: {}", error);
            return Err(Status::InternalServerError);
        }
    };

    if has_data {
        match Redis::get_data::<String, String>(r_conn, uri.clone()) {
            Ok(d) => {
                all_data = serde_json::from_str(&d).unwrap()
            }
            Err(error) => {
                error!(target:"routes/driver:list_all", "Error getting data from redis: {}", error);
                return Err(Status::InternalServerError);
            }
        }

    } else {
        // if not in cache we get the value and store in cache
        let connection = &mut establish_connection();
        all_data = match Driver::get_all_with_stats(connection) {
            Ok(driver_stats) => {
                AllData {
                    data_type: "drivers".to_string(),
                    table_data: TableData {
                        headers: vec![
                            "Driver".to_string(),
                            "Fastest Lap".to_string(),
                            "Median Laptime".to_string(),
                            "Total Laps".to_string(),
                            "Total Heats".to_string(),
                            "Rating".to_string(),
                            "".to_string(),
                        ],
                        rows: driver_stats
                            .iter()
                            .map(|stats| {
                                vec![
                                    stats.name.clone(),
                                    Math::round_float_to_n_decimals(stats.fastest_lap_time, 2).to_string(),
                                    Math::round_float_to_n_decimals(stats.median_lap_time, 2).to_string(),
                                    stats.total_laps.to_string(),
                                    stats.total_heats.to_string(),
                                    Math::round_float_to_n_decimals(stats.rating, 2).to_string(),
                                ]
                            })
                            .collect(),
                    },
                }
            }
            Err(_) => {
                return Err(Status::InternalServerError);
            }
        };




        // add to cache
        cache_data_to_url!(all_data, uri, "routes/driver:list_all");
    }


    Ok(Template::render(
        "all",
        all_data
    ))
}

#[get("/<driver_name>")]
pub fn single(driver_name: String, origin: &Origin) -> Result<Template, Status> {
    let sanitized = sanitize_name(&driver_name);
    if sanitized != driver_name {
        return Err(Status::BadRequest)
    }

    let r_conn_m = &mut Redis::connect();
    if r_conn_m.is_err() {
        error!(target:"routes/driver:single", "Error connecting to redis");
        return Err(Status::InternalServerError);
    }
    let r_conn = &mut r_conn_m.as_mut().unwrap();

    let uri = origin.path().to_string();
    let all_data;

    let has_data = match Redis::has_data(r_conn, uri.clone()) {
        Ok(b) => b,
        Err(error) => {
            error!(target:"routes/driver:list_all", "Error checking redis for data: {}", error);
            return Err(Status::InternalServerError);
        }
    };

    if has_data {
        all_data = serde_json::from_str(&Redis::get_data::<String, String>(r_conn, uri.clone()).unwrap()).unwrap()
    } else {

        let conn = &mut establish_connection();

        let driver = db_handle_get_error_http!(Driver::get_by_name(conn, &driver_name), "routes/driver:single", "driver");
        let laps = db_handle_get_error_http!(driver.get_laps(conn), "routes/driver:single", "laps");
        let heats = db_handle_get_error_http!(Heat::from_laps(conn, &laps), "routes/driver:single", "heats");
        let karts = db_handle_get_error_http!(Kart::get_all(conn), "routes/driver:single", "karts");

        let mut datasets = Vec::new();
        let mut table_rows: Vec<Vec<String>> = Vec::new();
        let laps_per_heat = Heat::get_laps_per_heat(&heats, &laps);

        for (heat, laps) in laps_per_heat {
            let kart = karts[..]
                .iter()
                .find(|e| e.id == laps[0].kart_id)
                .unwrap();

            let laps_statistics = Lap::get_stats_of_laps(&laps);

            let fastest_lap: Lap =
                Lap::find_laptime_in_laps(laps_statistics.fastest_lap_time.clone(), &laps).unwrap();

            table_rows.push(generate_data_rows(
                &heat,
                &laps,
                laps_statistics,
                &fastest_lap,
                kart,
            ));
            datasets.push(generate_data_set(&heat, &laps, driver.clone()));
        }

        all_data = SingleResponse {
            name: driver.name.to_string(),
            chart_data: ChartData {
                labels: datasets.iter().map(|e| e.label.clone()).collect(),
                datasets,
            },
            table_data: TableData {
                headers: vec![
                    "Heat".to_string(),
                    "Kart".to_string(),
                    "Average Laptime".to_string(),
                    "Median Laptime".to_string(),
                    "Fastest Laptime".to_string(),
                    "Fastest Lap".to_string(),
                    "Total Laps".to_string(),
                    "Date".to_string(),
                ],
                rows: table_rows,
            },
        };

        cache_data_to_url!(all_data, uri, "routes/driver:single");
    }

    Ok(Template::render(
        "driver",
        all_data
    ))
}

fn generate_data_rows(
    heat: &Heat,
    heat_laps: &[Lap],
    laps_stats: LapsStats,
    fastest_lap: &Lap,
    kart: &Kart,
) -> Vec<String> {
    vec![
        format!("<a href=\"/heats/{}\"> {}</a>", heat.heat_id, heat.heat_id),
        format!("<a href=\"/karts/{}\"> {}</a>", kart.number, kart.number),
        Math::round_float_to_n_decimals(laps_stats.median_lap_time, 2).to_string(),
        Math::round_float_to_n_decimals(laps_stats.avg_lap_time, 2).to_string(),
        Math::round_float_to_n_decimals(fastest_lap.lap_time.clone(), 2).to_string(),
        fastest_lap.lap_in_heat.to_string(),
        heat_laps.len().to_string(),
        heat.start_date.to_string(),
    ]
}

fn generate_data_set(heat: &Heat, heat_laps: &[Lap], driver: Driver) -> ChartDataDataset {
    ChartDataDataset {
        label: heat.heat_id.to_string(),
        data: heat_laps
            .iter()
            .map(|lap| ChartDataDataSetData {
                date: Some(heat.start_date.date()),
                driver: Some(driver.clone()),
                lap_time: lap.lap_time,
            })
            .collect(),
    }
}
#[derive(Serialize, Clone, Deserialize)]
struct SingleResponse {
    pub name: String,
    pub chart_data: ChartData,
    pub table_data: TableData,
}
