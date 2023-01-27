use std::time::Duration;
use log::{info, warn};
use tokio::task::JoinSet;
use tokio_cron_scheduler::{Job, JobScheduler};
use crate::errors::{CustomResult, Error};


use crate::modules::heat_api::{get_heat_from_api, get_todays_heats_from_api, save_heat};
use crate::modules::models::general::establish_connection;

pub async fn load_todays_heats() {

    let heat_list: Vec<String> = get_todays_heats_from_api().await;
    let mut tasks = JoinSet::new();

    for heat_id in heat_list {
        tasks.spawn(async move {
            println!("loading heat: {}", heat_id);
            let heat = match get_heat_from_api(heat_id).await {
                Ok(heat) => heat,
                Err(err) => {
                    warn!(target:"load_todays_heats", "failed loading heat from api. (heat_id: {})", err);
                    return;
                }
            };

            let con = &mut establish_connection();
            match save_heat(con, heat.clone()) {
                Ok(heat_id) => {
                    info!(target:"load_todays_heats", "saved heat: {}", heat_id);
                }
                Err(Error::AlreadyExistsError{ .. }) => {
                    info!(target:"load_todays_heats", "heat already exists: {}", heat.heat.id);
                }
                Err(Error::InvalidNameError{ .. }) => {
                    warn!(target:"load_todays_heats", "invalid driver names in heat {}", heat.heat.id);
                }
                _ => {}
            };
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
                load_todays_heats().await;
            })

        },
    ).unwrap();
    scheduler.add(j).await.unwrap();
    scheduler.start().await.unwrap();
}


