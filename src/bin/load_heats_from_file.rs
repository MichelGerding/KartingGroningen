use dotenvy::dotenv;

use karting_groningen_analytics::modules::heat_api::{get_heat_from_api, save_heat};
use karting_groningen_analytics::modules::helpers::heat::HeatsHelper;
use karting_groningen_analytics::modules::models::general::establish_connection;

//q: how can i fix the error temporary value dropped while borrowed?
//a: https://stackoverflow.com/questions/30353462/temporary-value-dropped-while-borrowed

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();
    let connection = &mut establish_connection();

    let file_url = "./src/heats.txt";

    let heat_list: Vec<String> = HeatsHelper::load_heat_ids_from_file(file_url);
    // let heats: Vec<WebResponse> = get_heats_from_api(heat_list).await;

    for heat_id in heat_list {
        let heat = get_heat_from_api(heat_id).await;

        save_heat(connection, heat).expect("error saving heat");
    }
}
