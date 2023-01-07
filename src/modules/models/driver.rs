use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::dsl::exists;
use diesel::select;
use crate::models;

use serde::{Deserialize, Serialize};
use crate::modules::models::lap::Lap;
use crate::schema::*;



#[derive(Queryable, Serialize, Identifiable, PartialEq, Debug, Clone, Deserialize)]
pub struct Driver {
    pub id: i32,
    pub name: String,
    pub fastest_lap: Option<i32>,
}

impl Driver {
    pub fn new(conn: &mut PgConnection, name: &String) -> Driver {
        let new_driver = models::NewDriver { name: name.clone() };

        diesel::insert_into(drivers::table)
            .values(&new_driver)
            .get_result(conn)
            .expect("Error saving new driver")
    }

    pub fn exists(conn: &mut PgConnection, name_in: &String) -> bool {
        use crate::schema::drivers::dsl::*;
        select(exists(drivers.filter(name.like(name_in))))
            .get_result(conn)
            .unwrap()
    }

    pub fn get_by_name(conn: &mut PgConnection, name_in: &String) -> Driver {
        use crate::schema::drivers::dsl::*;
        drivers
            .filter(name.like(name_in))
            .first::<Driver>(conn)
            .expect("Error loading drivers")
    }

    pub fn get_by_id(conn: &mut PgConnection, id_in: i32) -> Driver {
        use crate::schema::drivers::dsl::*;
        drivers
            .filter(id.eq(id_in))
            .first::<Driver>(conn)
            .expect("Error loading drivers")
    }

    pub fn ensure_exists(conn: &mut PgConnection, name: &String) -> Driver {
        if !Driver::exists(conn, name) {
            Driver::new(conn, name)
        } else {
            Driver::get_by_name(conn, name)
        }
    }


    pub fn from_laps(conn: &mut PgConnection, laps: &Vec<Lap>) -> Vec<Driver> {
        use crate::schema::drivers::dsl::*;
        let mut driver_ids = Vec::new();
        for lap in laps {
            driver_ids.push(lap.driver);
        }
        drivers
            .filter(id.eq_any(driver_ids))
            .load::<Driver>(conn)
            .unwrap()
    }

    pub fn to_new(&self) -> models::NewDriver {
        models::NewDriver {
            name: self.name.clone(),
        }
    }
}
