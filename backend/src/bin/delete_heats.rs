use karting_groningen_analytics::modules::helpers::logging::setup_logging;
use log::info;
use karting_groningen_analytics::modules::database::models::session::Session;

#[tokio::main]
async fn main() {
    setup_logging().expect("Error setting up logging");

    let heats = [];
    for heat_id in heats {
        let heat = Session::delete_id(heat_id).await;
        info!(target:"delete_heat", "Deleted heat: {:?}", heat);
    }
}
