use karting_groningen_analytics::modules::helpers::logging::setup_logging;
use karting_groningen_analytics::modules::models::session::Session;
use log::info;
use rocket::http::private::cookie::Expiration::Session;

#[tokio::main]
pub async fn main() {
    setup_logging().expect("Error setting up logging");

    let heats = Session::get_all_chronologicaly().await;
    for heat in heats {
        info!(target:"apply_ratings", "applying ratings of heat: {} ", heat.heat_id);
        heat.apply_ratings().await;
    }
}
