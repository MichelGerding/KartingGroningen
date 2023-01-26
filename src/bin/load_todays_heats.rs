use tokio::task::JoinSet;
use karting_groningen_analytics::modules::heat_api::{get_heat_from_api, get_todays_heats_from_api, save_heat};
use karting_groningen_analytics::modules::models::general::establish_connection;

#[tokio::main]
pub async fn main() {
    let heat_list: Vec<String> = get_todays_heats_from_api().await;
    let mut tasks = JoinSet::new();

    for heat_id in heat_list {
        tasks.spawn(async move {
            println!("loading heat: {}", heat_id);
            let heat = get_heat_from_api(heat_id).await;

            let con = &mut establish_connection();
            save_heat(con, heat).expect("failed to save heat");
        });
    }

    while let Some(heat) = tasks.join_next().await {
        heat.unwrap();
    }
}