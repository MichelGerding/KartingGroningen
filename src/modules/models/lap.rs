use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models;
use crate::models::{NewDriver, NewLap};
use crate::modules::models::driver::Driver;
use crate::modules::models::heat::Heat;
use crate::modules::models::kart::Kart;

use crate::schema::laps;


#[derive(Queryable, Serialize, Associations, Identifiable, PartialEq, Debug, Clone, Deserialize)]
#[diesel(belongs_to(Heat, foreign_key = heat))]
#[diesel(belongs_to(Driver, foreign_key = driver))]
#[diesel(belongs_to(Kart, foreign_key = kart_id))]
pub struct Lap {
    pub id: i32,
    pub heat: i32,
    pub driver: i32,
    pub lap_in_heat: i32,
    pub lap_time: f64,
    pub kart_id: i32,
}

impl Lap {
    pub fn new(
        conn: &mut PgConnection,
        heat: i32,
        driver: i32,
        lap_in_heat: i32,
        lap_time: f64,
        kart_id: i32,
    ) -> Lap {
        let new_lap = models::NewLap {
            heat,
            driver,
            lap_in_heat,
            lap_time,
            kart_id,
        };

        diesel::insert_into(laps::table)
            .values(&new_lap)
            .get_result(conn)
            .expect("Error saving new lap")
    }

    pub fn get_laps_belonging_to_heat(conn: &mut PgConnection, heat: &Heat) -> Vec<Lap> {
        Lap::belonging_to(heat)
            .load::<Lap>(conn)
            .unwrap()
    }

    pub fn from_kart(conn: &mut PgConnection, kart: &Kart) -> Vec<Lap> {
        Lap::belonging_to(kart)
            .load::<Lap>(conn)
            .unwrap()
    }



    pub fn to_new(&self) -> NewLap {
        NewLap {
            heat: self.heat,
            driver: self.driver,
            lap_in_heat: self.lap_in_heat,
            lap_time: self.lap_time,
            kart_id: self.kart_id,
        }
    }


}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LapDriver {
    pub lap: NewLap,
    pub driver: NewDriver,
}
