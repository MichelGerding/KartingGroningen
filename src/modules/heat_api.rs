use diesel::PgConnection;
use serde::{Deserialize};

use log::{info, warn};
use crate::modules::models::driver::Driver;
use crate::modules::models::heat::Heat;
use crate::modules::models::kart::Kart;
use crate::modules::models::lap::Lap;


pub async fn get_heats_from_api(heat_ids: Vec<String>) -> Vec<WebResponse> {
    let mut heats: Vec<WebResponse> = Vec::new();

    //TODO:: make fully async
    for heat_id in heat_ids {
        heats.push(get_heat_from_api(heat_id).await);
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



pub async fn get_heat_from_api(heat_id: String) -> WebResponse {
    info!(target: "querying_heat", "Getting heat {} from api", heat_id);
    let request_url = format!("http://reserveren.kartbaangroningen.nl/GetHeatResults.ashx?heat={heat_id}");
    let response = reqwest::get(&request_url).await.unwrap();

    let body = response.text().await.unwrap();

    // clean response string
    let mut body_cleaned = body.replace('(', "");
    body_cleaned = body_cleaned.replace(");", "");

    serde_json::from_str(&body_cleaned).unwrap()
}

pub fn save_heat(conn: &mut PgConnection, heat: WebResponse) -> Result<(), &'static str> {

    if Heat::exists(conn, &heat.heat.id) {
        warn!(target: "saving_heat", "heat already exists, skipping {}", heat.heat.id);
        return Ok(());
    }

    let heat_id = Heat::new(conn, &heat.heat.id, &heat.heat.heat_type_name, &heat.heat.start_time);
    for driver in heat.results {

        let driver_id = Driver::ensure_exists(conn, &driver.participation.driver_name.to_string());
        let kart_id = Kart::ensure_exists(conn,driver.result.kart_nr,None);

        let mut lap_in_heat = 0;
        for lap in driver.result.lap_times {
            lap_in_heat += 1;
            Lap::new(
                conn,
                heat_id.id,
                driver_id.id,
                lap_in_heat,
                lap,
                kart_id.id);
        }
    }


    info!(target: "saving_heat", "heat {} saved", heat.heat.id);
    Ok(())
}

#[derive(Deserialize, Debug)]
struct HeatsList {
    #[serde(rename = "Results")]
    pub heats: Vec<HeatId>,
}

#[derive(Deserialize, Debug)]
struct HeatId{
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct ParticipationInfo {
    #[serde(rename = "driverName")]
    pub driver_name: String,
}

#[derive(Debug, Deserialize)]
pub struct ResultInfo {
    #[serde(rename = "KartNr")]
    pub kart_nr: i32,
    #[serde(rename = "LapTimes")]
    pub lap_times: Vec<f64>,
}

#[derive(Debug, Deserialize)]
pub struct HeatResult {
    #[serde(rename = "Participation")]
    pub participation: ParticipationInfo,
    #[serde(rename = "Result")]
    pub result: ResultInfo,
}

#[derive(Debug, Deserialize)]
pub struct WebResponse {
    #[serde(rename = "Heat")]
    pub heat: HeatInfo,
    #[serde(rename = "Results")]
    pub results: Vec<HeatResult>,
}
