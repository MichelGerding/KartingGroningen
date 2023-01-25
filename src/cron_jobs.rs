use std::time::Duration;
use tokio::task::JoinSet;
use tokio_cron_scheduler::{Job, JobScheduler};


use crate::modules::heat_api::{get_heat_from_api, get_todays_heats_from_api, save_heat};
use crate::modules::models::general::establish_connection;

async fn load_heats() {
    let heat_list: Vec<String> = get_todays_heats_from_api().await;
    let mut tasks = JoinSet::new();

    for heat_id in heat_list {
        tasks.spawn(async move {
            let heat = match get_heat_from_api(heat_id).await {
                Ok(heat) => heat,
                Err(err) => {
                    println!("Error: {}", err);
                    return;
                }
            };

            let con = &mut establish_connection();
            save_heat(con, heat).expect("failed to save heat");
        });
    }

    while let Some(heat) = tasks.join_next().await {
        heat.unwrap();
    }
}


pub async fn register_cron_jobs() {
    let scheduler = JobScheduler::new().await.unwrap();

    // run every 2 hours
    let j = Job::new_repeated_async(
        Duration::from_secs( 7200), // 2 hours
        |_uuid, _l| {
            Box::pin(async {
                load_heats().await;
            })

        },
    ).unwrap();
    scheduler.add(j).await.unwrap();
    scheduler.start().await.unwrap();
}


