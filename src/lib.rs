use crate::modules::models::driver::Driver;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub mod errors;
pub mod modules;
pub mod schema;
pub mod cron_jobs;

pub mod macros {
    pub mod request_caching;
    pub mod database_error_handeler;
    pub mod redis;
}

pub mod routes {
    pub mod api {
        pub mod driver;
        pub mod heat;
        pub mod kart;
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TemplateDataHeat {
    pub heat_id: String,
    pub heat_type: String,
    pub start_date: chrono::NaiveDateTime,
    pub chart_data: ChartData,
    pub table_data: TableData,
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub datasets: Vec<ChartDataDataset>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChartDataDataset {
    pub label: String,
    pub data: Vec<ChartDataDataSetData>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChartDataDataSetData {
    pub date: Option<NaiveDate>,
    pub driver: Option<Driver>,
    pub lap_time: f64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AllData {
    pub data_type: String,
    pub table_data: TableData,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TableData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AllDataWithCharts {
    pub data_type: String,
    pub chart_data: ChartData,
    pub table_data: TableData,
}
