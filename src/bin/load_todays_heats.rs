use log::info;
use karting_groningen_analytics::modules::models::general::establish_connection;
use karting_groningen_analytics::modules::heat_api::{get_heats_from_api, get_todays_heats_from_api, save_heat, WebResponse};



#[tokio::main]
async fn main() {
    env_logger::init();

    let connection = &mut establish_connection();

    info!(target: "heats", "Getting todays heats from API: date={}", chrono::Local::now().naive_local());
    let heat_list: Vec<String> = get_todays_heats_from_api().await;
    let heats: Vec<WebResponse> = get_heats_from_api(heat_list).await;

    for heat in heats {
        save_heat(connection, heat).expect("Failed to save heat");
    }
}
