use log::{info};
use karting_groningen_analytics::modules::helpers::logging::setup_logging;
use karting_groningen_analytics::modules::models::general::establish_connection;
use karting_groningen_analytics::modules::models::heat::Heat;

pub fn main() {
    setup_logging().expect("Error setting up logging");

    let connection = &mut establish_connection();

    let heats = Heat::get_all_chronologicaly(connection);
    for heat in heats {
        info!(target:"apply_ratings", "applying ratings of heat: {} ", heat.heat_id);
        let _ = heat.apply_ratings(connection);
    }
}