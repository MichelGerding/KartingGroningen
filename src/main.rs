mod schema;


use rocket::{Build, Rocket};
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::{Template};
use karting_groningen_analytics::modules::helpers::handelbars::format_date::FormatDateHelper;
use karting_groningen_analytics::modules::helpers::handelbars::format_heat_type::FormatHeatType;
use karting_groningen_analytics::modules::helpers::handelbars::format_is_child_kart::FormatChildKart;

use karting_groningen_analytics::modules::helpers::handelbars::get_laps_at_time::GetLapsAtTimeHelper;

#[macro_use] extern crate rocket;

use karting_groningen_analytics::routes::{heat, kart};

#[get("/")]
fn index() -> Template {
    Template::render("index", std::collections::HashMap::<String, String>::new())
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper("getLapsAtTime", Box::new(GetLapsAtTimeHelper));
            engines.handlebars.register_helper("formatDate", Box::new(FormatDateHelper));
            engines.handlebars.register_helper("formatHeatType", Box::new(FormatHeatType));
            engines.handlebars.register_helper("formatChildKart", Box::new(FormatChildKart));
            engines.handlebars.set_strict_mode(true);
        }))
        .mount("/heats", routes![
            heat::list,
            heat::single,
        ])
        .mount("/karts", routes![
            kart::list_all,
            kart::single,
        ])
        .mount("/", routes![index])
        .mount("/static", FileServer::from(relative!("static")))
}

