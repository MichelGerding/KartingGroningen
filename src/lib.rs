use serde::{Serialize, Deserialize};

pub mod models;

pub mod schema;
pub mod modules;
pub mod routes {
    pub mod heat;
    pub mod kart;
}




#[derive(Clone, Serialize, PartialEq, Deserialize, Debug)]
pub struct TemplateData {
    pub heat_id: String,
    pub heat_type: String,
    pub start_date: chrono::NaiveDateTime,
    pub drivers: Vec<TemplateDataDriver>,
}
#[derive(Clone, Serialize, PartialEq, Deserialize, Debug)]
pub struct TemplateDataDriver {
    pub driver_name: String,
    pub fastest_lap: TemplateDataLap,
    pub total_laps: usize,
    pub all_laps: Vec<TemplateDataLap>,
    pub outlier_laps: Vec<TemplateDataLap>,
    pub normal_laps: Vec<TemplateDataLap>,
    pub kart: i32,
    pub avg_lap_time: f64,
}

#[derive(Clone, Serialize, PartialEq, Deserialize, Debug)]
pub struct TemplateDataLap {
    pub lap_in_heat: i32,
    pub lap_time: f64,
}
