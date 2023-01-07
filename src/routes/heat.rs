use rocket::{get};
use rocket_dyn_templates::{context, Template};

use serde_json::to_string;

use crate::{TemplateData};
use crate::modules::models::general::establish_connection;
use crate::modules::models::lap::Lap;
use crate::modules::helpers::driver::{DriverHelpers};
use crate::modules::models::driver::Driver;

use crate::models::{NewHeat};
use crate::modules::models::heat::Heat;




#[get("/all")]
pub fn list() -> Template {
    let connection = &mut establish_connection();
    let results: Vec<Heat> = Heat::get_all(connection);

    let mut results_vec: Vec<NewHeat> = Vec::new();
    for result in &results {
        results_vec.push(NewHeat {
            heat_id: result.heat_id.clone(),
            heat_type: result.heat_type.clone(),
            start_date: result.start_date,
        })
    }

    let context = context! {
        results: &results_vec,
    };

    Template::render("all_heats", context)
}

#[get("/single/<heat_id_in>")]
pub fn single(heat_id_in: String) -> Template {
    let conn = &mut establish_connection();

    // get all data needed
    let heat_res = Heat::get_by_id(conn, &heat_id_in);
    let v_laps = Lap::get_laps_belonging_to_heat(conn, &heat_res);
    let v_driver: Vec<Driver> = Driver::from_laps(conn, &v_laps);

    let mut template_data: TemplateData = TemplateData {
        heat_id: heat_res.heat_id,
        heat_type: heat_res.heat_type,
        start_date: heat_res.start_date,
        drivers: Vec::new(),
    };

    // get stats for each driver
    for driver in &v_driver {
        template_data.drivers.push(DriverHelpers::get_stats_for_laps(conn, driver, &v_laps));
    }

    // create a context to pass the data to the template as object and as json string
    let context = context! {
        data: &template_data,
        json_string: to_string(&template_data).unwrap(),
    };

    Template::render("heat", context)
}
