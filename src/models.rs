use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::*;


#[derive(Insertable, Serialize, Debug, Clone, Deserialize)]
#[diesel(table_name = drivers)]
pub struct NewDriver<> {
    pub name: String
}

#[derive(Insertable, Serialize, Debug, Clone, Deserialize)]
#[diesel(table_name = heats)]
pub struct NewHeat {
    pub heat_id: String,
    pub heat_type: String,
    pub start_date: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Debug, Clone, Deserialize)]
#[diesel(table_name = karts)]
pub struct NewKart {
    pub number: i32,
    pub is_child_kart: Option<bool>,
}

#[derive(Insertable, Serialize, Debug, Clone, Deserialize)]
#[diesel(table_name = laps)]
pub struct NewLap {
    pub heat: i32,
    pub driver: i32,
    pub lap_in_heat: i32,
    pub lap_time: f64,
    pub kart_id: i32,
}
