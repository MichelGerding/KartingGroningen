use diesel::{Identifiable, PgConnection, Queryable};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::NewHeat;
use crate::schema::*;

#[derive(Queryable, Serialize, Identifiable, PartialEq, Debug, Clone, Deserialize)]
pub struct Heat {
    pub id: i32,
    pub heat_id: String,
    pub heat_type: String,
    pub start_date: chrono::NaiveDateTime,
}

impl Heat {

    pub fn new(conn: &mut PgConnection, _heat_id: &str, heat_type_in: &str, other_start_date: &str) -> Heat {
        use crate::schema::heats::dsl::*;


        let timestamp = NaiveDateTime::parse_from_str(other_start_date, "%Y-%m-%dT%H:%M:%S%.f%z").unwrap();
        let new_heat = NewHeat {
            heat_id: _heat_id.to_string(),
            heat_type: heat_type_in.to_string(),
            start_date: timestamp,
        };

        diesel::insert_into(heats)
            .values(&new_heat)
            .get_result(conn)
            .expect("Error saving new heat")
    }

    pub fn exists(conn: &mut PgConnection, heat_id_in: &str) -> bool {
        use diesel::select;
        use diesel::dsl::exists;
        use crate::schema::heats::dsl::*;

        select(exists(heats.filter(heat_id.like(heat_id_in))))
            .get_result(conn)
            .unwrap()
    }

    pub fn get_by_db_id(conn: &mut PgConnection, heat_id_in: i32) -> Heat {
        use crate::schema::heats::dsl::*;

        heats
            .find(heat_id_in)
            .first::<Heat>(conn)
            .expect("Error loading heat")
    }

    pub fn get_by_id(conn: &mut PgConnection, heat_id_in: &str) -> Heat {
        use crate::schema::heats::dsl::*;

        heats
            .filter(heat_id.like(heat_id_in))
            .first::<Heat>(conn)
            .expect("Error loading heat")
    }

    pub fn get_all(conn: &mut PgConnection) -> Vec<Heat> {
        use crate::schema::heats::dsl::*;

        heats
            .load::<Heat>(conn)
            .expect("Error loading heats")
    }

    pub fn ensure_exists(conn: &mut PgConnection, _heat_id: &str, heat_type: &str, _start_time: &str) -> Heat {
        if !Heat::exists(conn, _heat_id) {
            Heat::new(conn, _heat_id, heat_type, _start_time)
        } else {
            Heat::get_by_id(conn, _heat_id)
        }
    }

    pub fn to_new(&self) -> NewHeat {
        NewHeat {
            heat_id: self.heat_id.clone(),
            heat_type: self.heat_type.clone(),
            start_date: self.start_date.clone(),
        }
    }
}
