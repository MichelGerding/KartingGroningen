use std::collections::HashMap;

use chrono::NaiveDate;
use diesel::dsl::exists;
use diesel::pg::{Pg, PgConnection};
use diesel::prelude::*;
use diesel::select;
use serde::Serialize;

use crate::models;
use crate::modules::helpers::math::Math;
use crate::modules::models::driver::Driver;
use crate::modules::models::heat::Heat;
use crate::modules::models::lap::{Lap, LapDriver};
use crate::schema::karts;
use crate::schema::karts::{id, number};

#[derive(Queryable, Serialize, Identifiable, PartialEq, Debug, Clone)]
pub struct Kart {
    pub id: i32,
    pub number: i32,
    pub is_child_kart: Option<bool>,
}

impl Kart {
    pub fn new(conn: &mut PgConnection, _number: i32, is_child_kart: Option<bool>) -> Kart {
        let new_kart = models::NewKart {
            number: _number,
            is_child_kart,
        };

        diesel::insert_into(karts::table)
            .values(&new_kart)
            .get_result(conn)
            .expect("Error saving new kart")
    }

    pub fn exists(conn: &mut PgConnection, number_in: i32) -> bool {
        use crate::schema::karts::dsl::karts;
        select(exists(karts.filter(number.eq(number_in)))).get_result(conn).unwrap()
    }

    pub fn get_by_number(conn: &mut PgConnection, number_in: i32) -> Kart {
        use crate::schema::karts::dsl::karts;
        karts
            .filter(number.eq(number_in))
            .first::<Kart>(conn)
            .expect("Error loading kart")
    }

    pub fn get_by_id(conn: &mut PgConnection, id_in: i32) -> Kart {
        use crate::schema::karts::dsl::karts;
        karts
            .filter(id.eq(id_in))
            .first::<Kart>(conn)
            .expect("Error loading drivers")
    }
    pub fn get_all(conn: &mut PgConnection) -> Vec<Kart> {
        use crate::schema::karts::dsl::karts;
        karts
            .load::<Kart>(conn)
            .expect("Error loading karts")
    }

    pub fn get_laps_driver_and_heat(&self, conn: &mut PgConnection) -> Vec<LapDriver> {
        use crate::schema::heats::dsl::{heats, start_date};
        use crate::schema::laps::dsl::*;

        let v_laps: Vec<Lap> =
            Lap::belonging_to(self)
                .inner_join(heats)
                .order((start_date, lap_in_heat))
                .select((id, heat, driver, lap_in_heat, lap_time, kart_id))
                .load(conn)
                .unwrap();

        v_laps
            .iter()
            .map(|lap| {
                LapDriver {
                    lap: lap.to_new(),
                    driver: Driver::get_by_id(conn, lap.driver).to_new(),
                }
            }).collect()
    }

    pub fn get_laps_per_day(&self, conn: &mut PgConnection) -> HashMap<NaiveDate, Vec<f64>> {
        let v_laps: Vec<Lap>;
        {
            use crate::schema::heats::dsl::{heats, start_date};
            use crate::schema::laps::dsl::*;

            v_laps = Lap::belonging_to(self)
                .inner_join(heats)
                .order((start_date, lap_in_heat))
                .select((id, heat, driver, lap_in_heat, lap_time, kart_id))
                .load(conn)
                .unwrap();
        }

        let mut laps_per_day: HashMap<NaiveDate, Vec<f64>> = HashMap::new();
        for lap in &v_laps {
            let heat = Heat::get_by_db_id(conn, lap.heat);
            let date = heat.start_date.date();
            let laptime = lap.lap_time;

            if let std::collections::hash_map::Entry::Vacant(e) = laps_per_day.entry(date) {
                e.insert(vec![laptime]);
            } else {
                laps_per_day.get_mut(&date).unwrap().push(laptime);
            }
        }

        laps_per_day
    }

    pub fn get_minimum_laptime_per_day(&self, conn: &mut PgConnection) -> HashMap<NaiveDate, f64> {
        let days = self.get_laps_per_day(conn);
        days
            .iter()
            .map(|(date, laps)| {
                let minimum = laps.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                (*date, Math::round_float_to_n_decimals(minimum, 2))
            }).collect()
    }

    pub fn get_median_laptime_per_day(&self, conn: &mut PgConnection) -> HashMap<NaiveDate, f64> {
        let days = self.get_laps_per_day(conn);
        days
            .iter()
            .map(|(date, laps)| {
                let median = Math::median(laps.clone());
                (*date, Math::round_float_to_n_decimals(median, 2))
            }).collect()
    }

    pub fn get_laps_avg_per_day(&self, conn: &mut PgConnection) -> HashMap<NaiveDate, f64> {
        let avg_day = self.get_laps_per_day(conn);
        avg_day
            .iter()
            .map(|(date, laps)| {
                let avg = laps.iter().sum::<f64>() / laps.len() as f64;
                (*date, Math::round_float_to_n_decimals(avg, 2))
            }).collect()
    }
    
    pub fn ensure_exists(conn: &mut PgConnection, _number: i32, is_child_kart: Option<bool>) -> Kart {
        if !Kart::exists(conn, _number) {
            Kart::new(conn, _number, is_child_kart)
        } else {
            Kart::get_by_number(conn, _number)
        }
    }
}
