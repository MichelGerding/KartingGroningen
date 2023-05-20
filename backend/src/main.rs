use karting_groningen_analytics::cron_jobs::register_cron_jobs;
use karting_groningen_analytics::modules::helpers::rocket_fairings::cors::CORS;
use rocket::{launch, routes, Build, Rocket};

// use karting_groningen_analytics::cron_jobs::{load_heat_cron, register_cron_jobs};

use karting_groningen_analytics::modules::helpers::logging::setup_logging;
use karting_groningen_analytics::routes::api;
use task_queue;
use karting_groningen_analytics::modules::database::models::general::setup_pool;

#[launch]
async fn rocket() -> Rocket<Build> {

    setup_logging().expect("Failed to setup logging");

    setup_pool().await;
    // start the task thread
    task_queue::setup();

    // register cron jobs that need to run.
    // these are jobs that either need to effect the database, redis, or both.
    register_cron_jobs().await;

    // start the webserver
    rocket::build().attach(CORS).mount(
        "/api",
        routes![
            // heats
            api::heat::save_one,
            // api::heat::delete,
            api::heat::get_one,
            api::heat::get_all_ids,
            api::heat::get_one_stats,
            api::heat::search,
            // driver
            api::driver::search,
            api::driver::search_full,
            api::driver::get_one_stats,
            api::driver::get_one,
            api::driver::get_all_ids,
            //kart
            api::kart::get_one,
            api::kart::get_all,
            api::kart::get_one_full,
        ],
    )
}

//TODO:: add caching to page and api requests.
//          add caching to search urls
//          clear cache for search urls
//TODO:: create a nice looking and interactive ui
//          add scripting testability to ui
//          add ability to create custom dashboards
//TODO:: add caching to all db queries
//TODO:: add ranking to the drivers
//TODO:: add support for different tracks
