use dotenvy::dotenv;
use karting_groningen_analytics::errors::Error;
use log::{error, info, warn};

use karting_groningen_analytics::modules::heat_api::{get_heat_from_api, save_heat};
use karting_groningen_analytics::modules::helpers::heat::HeatsHelper;
use karting_groningen_analytics::modules::helpers::logging::setup_logging;
use karting_groningen_analytics::modules::database::models::general::setup_pool;

#[tokio::main]
async fn main() {
    dotenv().ok();
    setup_logging().expect("failed to setup logging");
    setup_pool().await;

    // get all the heats stored in the file
    let file_url = "./src/heats.txt";
    let heat_list: Vec<String> = match HeatsHelper::load_heat_ids_from_file(file_url) {
        Ok(heats) => heats,
        Err(Error::FileDoesNotExistError {}) => {
            error!(target:"load_files-From_heat", "File does not exist: {}", file_url);
            return;
        }
        Err(Error::PermissionDeniedError {}) => {
            error!(target:"load_files-From_heat", "Permission denied: {}", file_url);
            return;
        }
        _ => unreachable!("Unexpected error"),
    };

    // get the info from the heats and save into database
    for heat_id in heat_list {
        match get_heat_from_api(heat_id.clone()).await {
            Ok(heat) => match save_heat(heat).await {
                Ok(_) => {
                    info!(target:"load_heats_from_file", "saved heat: {}", heat_id);
                }
                Err(Error::AlreadyExistsError { .. }) => {
                    info!(target:"load_heats_from_file", "heat already exists: {}", heat_id);
                }
                Err(Error::InvalidNameError { .. }) => {
                    warn!(target:"load_heats_from_file", "invalid driver names in heat {}", heat_id);
                }
                _ => {
                    unreachable!()
                }
            },
            Err(err) => {
                error!(target:"load_heats_from_file", "failed loading heat from api. (heat_id: {})", err);
            }
        };
    }
}
