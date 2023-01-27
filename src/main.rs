mod schema;

use log::{info};
use karting_groningen_analytics::modules::helpers::handelbars::format::Format;
use rocket::fs::{relative, FileServer};
use rocket::{Build, Rocket, get, launch, routes};
use rocket_dyn_templates::Template;
use karting_groningen_analytics::cron_jobs::register_cron_jobs;

// use karting_groningen_analytics::cron_jobs::{load_heat_cron, register_cron_jobs};

use karting_groningen_analytics::modules::helpers::handelbars::format_date::FormatDateHelper;
use karting_groningen_analytics::modules::helpers::handelbars::format_heat_type::FormatHeatType;
use karting_groningen_analytics::modules::helpers::handelbars::format_is_child_kart::FormatIsChildKartHandlebars;
use karting_groningen_analytics::modules::helpers::handelbars::to_json::ToJson;
use karting_groningen_analytics::modules::helpers::logging::setup_logging;

use karting_groningen_analytics::routes::{api, driver, heat, kart};



#[get("/")]
pub fn index() -> Template {

    info!("index");
    Template::render("index", ())
}

#[launch]
async fn rocket() -> Rocket<Build> {
    setup_logging().expect("Failed to setup logging");


    // register cron jobs that need to run.
    // these are jobs that either need to effect the database, redis, or both.
    register_cron_jobs().await;


    // start the webserver
    rocket::build()
        .attach(Template::custom(|engines| {
            // handlebar helper functions.
            // can be used as {{name <data>}}
            engines
                .handlebars
                .register_helper("format", Box::new(Format));
            engines
                .handlebars
                .register_helper("formatChildKart", Box::new(FormatIsChildKartHandlebars));
            engines
                .handlebars
                .register_helper("formatDate", Box::new(FormatDateHelper));
            engines
                .handlebars
                .register_helper("formatHeatType", Box::new(FormatHeatType));
            engines
                .handlebars
                .register_helper("toJson", Box::new(ToJson));

            // enforce strict checking for template.
            // makes it so we can only use data that is actually passed to the template.
            engines.handlebars.set_strict_mode(true);
        }))
        // all routes to be mounted.
        // list all we be at /all
        // single will be at /single/<id>
        // list_all_paginated will be at /all/<page>
        .mount("/heats", routes![heat::list_all, heat::single,])
        .mount("/karts", routes![kart::list_all, kart::single,])
        .mount("/drivers", routes![driver::list_all, driver::single,])
        .mount(
            "/api",
            routes![
                // heats
                api::heat::save_one,
                api::heat::delete,
                api::heat::get_one,
                api::heat::get_all,
                api::heat::get_all_ids,
                api::heat::get_one_stats,
                // driver
                api::driver::search,
                api::driver::search_full,
                api::driver::get_one_stats,
                api::driver::get_one,
                api::driver::get_all_ids,
                api::driver::get_all,
                //kart
                api::kart::get_one,
                api::kart::get_one_full,
                api::kart::get_all,
                api::kart::get_all_full,
            ],
        )
        .mount("/", routes![index,])
        // static files to be server
        // for example: images, scripts or stylesheets but not templates.
        .mount("/static", FileServer::from(relative!("front-end/static")))
}

//TODO:: add caching to page and api requests.
//          add caching to search urls
//          clear cache for search urls
//TODO:: create a nice looking and interactive ui
//          add paginated results to ui
//          add responsive ui which support the change of data.
//          add scripting testability to ui
//          add ability to create custom dashboards
//TODO:: add caching to all db queries
//TODO:: lower the amount of data required to show the pages
//TODO:: add ranking to the drivers
//TODO:: add propper logging
//TODO:: add support for different tracks
