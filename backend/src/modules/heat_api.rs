use serde::Deserialize;

use std::fmt::Debug;

use crate::errors::{CustomResult, Error};
use crate::modules::database::models::driver::{sanitize_name, Driver};
use crate::modules::database::models::session::Session;
use crate::modules::database::models::vehicle::Vehicle;
use crate::modules::database::models::lap::{Lap, NewLap};
use log::{info, warn};

use tokio::task::JoinSet;


pub async fn get_heats_from_api(heat_ids: Vec<String>) -> Vec<WebResponse> {
    let mut tasks = JoinSet::new();

    for heat_id in heat_ids {
        tasks.spawn(get_heat_from_api(heat_id));
    }

    let mut heats: Vec<WebResponse> = Vec::new();
    while let Some(heat) = tasks.join_next().await {
        let heat_result = match heat.unwrap() {
            Ok(heat) => heat,
            Err(err) => {
                warn!("Error: {}", err);
                continue;
            }
        };
        heats.push(heat_result);
    }

    heats
}

pub async fn get_todays_heats_from_api() -> Vec<String> {
    let mut heats: Vec<String> = Vec::new();
    let request_url = "http://reserveren.kartbaangroningen.nl/GetHeatResults.ashx";
    let response = reqwest::get(request_url).await.unwrap();

    let body = response.text().await.unwrap();

    let mut body_cleaned = body.replace('(', "");
    body_cleaned = body_cleaned.replace(");", "");

    let json: HeatsList = serde_json::from_str(&body_cleaned).unwrap();
    for heat in json.heats {
        heats.push(heat.id);
    }

    heats
}

pub async fn get_heat_from_api(heat_id: String) -> serde_json::Result<WebResponse> {
    info!(target: "modules/heat_api:querying_heat", "Getting heat {} from api", heat_id);
    let request_url =
        format!("http://reserveren.kartbaangroningen.nl/GetHeatResults.ashx?heat={heat_id}");
    let response = reqwest::get(&request_url).await.unwrap();

    let body = response.text().await.unwrap();


    // clean response string
    let mut body_cleaned = body.replace('(', "");
    body_cleaned = body_cleaned.replace(");", "");
    serde_json::from_str(&body_cleaned)
}

pub async fn save_heat(heat: WebResponse) -> CustomResult<String> {
    println!("0");

    if Session::exists(&heat.heat.id).await {
        return Err(Error::AlreadyExistsError {});
    }

    println!("1");

    // cleanup the name
    let mut name = heat.heat.heat_type_name.clone();
    let fullchars = "Grand Prix";
    println!("2");
    if name.contains("Gran") && !name.ends_with("x") {
        for i in 0..(fullchars.len() - 4) {
            // get the first name.len - 1 letters of the string
            let sub = &fullchars[0..fullchars.len() - i];

            if name.ends_with(sub) {
                let to_add = &fullchars[fullchars.len() - i..];

                name = format!("{}x{}", name, to_add);
                break;
            }
        }
    }
    println!("3");

    for driver in &heat.results {
        if driver.participation.driver_name.parse::<f64>().is_ok() {
            return Err(Error::InvalidNameError {});
        }
    }
    println!("4");

    let heat_id = Session::ensure_exists(&heat.heat.id, &*name, &heat.heat.start_time).await;
    println!("5");

    for driver in heat.results {
        let driver_name = sanitize_name(&driver.participation.driver_name);

        let kart = Vehicle::ensure_exists(
            driver.result.kart_nr,
            "RiM0".to_string(),
            "ALPHA2".to_string(),
            9,
            true).await;

        println!("5.1");
        let driver_id = Driver::ensure_exists(&driver_name).await;
        println!("5.2");

        let mut laps: Vec<NewLap> = Vec::new();

        let mut lap_in_heat = 0;
        for lap in driver.result.lap_times {
            lap_in_heat += 1;
            laps.push(NewLap {
                heat: heat_id.id,
                driver: driver_id.id,
                kart_id: kart.id,
                lap_in_heat: lap_in_heat as i32,
                lap_time: lap,
            });
        }

        let _ = Lap::insert_bulk(&laps).await;
        println!("5.3");

    }
    println!("6");

    heat_id.apply_ratings().await;
    println!("7");

    Ok(heat_id.heat_id)
}


#[derive(Deserialize, Debug)]
struct HeatsList {
    #[serde(rename = "Results")]
    pub heats: Vec<HeatId>,
}

#[derive(Deserialize, Debug)]
struct HeatId {
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HeatInfo {
    #[serde(rename = "JoinHeats")]
    pub join_heats: bool,
    #[serde(rename = "ParticipationCount")]
    pub participation_count: i32,
    pub id: String,
    #[serde(rename = "StartTime")]
    pub start_time: String,
    #[serde(rename = "HeatTypeName")]
    pub heat_type_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ParticipationInfo {
    #[serde(rename = "driverName")]
    pub driver_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ResultInfo {
    #[serde(rename = "KartNr")]
    pub kart_nr: i32,
    #[serde(rename = "LapTimes")]
    pub lap_times: Vec<f64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HeatResult {
    #[serde(rename = "Participation")]
    pub participation: ParticipationInfo,
    #[serde(rename = "Result")]
    pub result: ResultInfo,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WebResponse {
    #[serde(rename = "Heat")]
    pub heat: HeatInfo,
    #[serde(rename = "Results")]
    pub results: Vec<HeatResult>,
}
