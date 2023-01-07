use dotenvy::dotenv;

use karting_groningen_analytics::*;
use karting_groningen_analytics::modules::models::heat::Heat;
use karting_groningen_analytics::modules::models::general::establish_connection;
use karting_groningen_analytics::modules::heat_api::save_heat;
use karting_groningen_analytics::modules::helpers::heat::HeatsHelper;

use crate::modules::heat_api::{get_heats_from_api, WebResponse};

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();

    let connection = &mut establish_connection();

    let file_url = "/home/michel/Shared/Projects/karting_groningen_analytics/src/heats.txt";
    let heat_list: Vec<String> = HeatsHelper::load_heat_ids_from_file(file_url);
    let heats: Vec<WebResponse> = get_heats_from_api(heat_list).await;

    for heat in heats {
        if Heat::exists(connection, &heat.heat.id) {
            continue;
        }

        save_heat(connection, heat).expect("Failed to save heat");
    }
}
