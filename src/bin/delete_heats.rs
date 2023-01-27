use dotenvy::dotenv;
use log::info;
use karting_groningen_analytics::modules::helpers::logging::setup_logging;

use karting_groningen_analytics::modules::models::general::establish_connection;
use karting_groningen_analytics::modules::models::heat::Heat;

#[tokio::main]
async fn main() {
    setup_logging().expect("Error setting up logging");
    
    let connection = &mut establish_connection();

    let heats = [];

    for heat_id in heats {
        let heat = Heat::delete_id(connection, heat_id);
        info!(target:"delete_heat", "Deleted heat: {:?}", heat);
    }
}
