use karting_groningen_analytics::cron_jobs::load_todays_heats;
use karting_groningen_analytics::modules::helpers::logging::setup_logging;

#[tokio::main]
pub async fn main() {
    setup_logging().expect("Error setting up logging");

    load_todays_heats().await;
}