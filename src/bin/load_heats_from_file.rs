use dotenvy::dotenv;
use log::{error, info, warn};
use karting_groningen_analytics::errors::{AlreadyExistsError, CustomResult, Error};

use karting_groningen_analytics::modules::heat_api::{get_heat_from_api, save_heat};
use karting_groningen_analytics::modules::helpers::heat::HeatsHelper;
use karting_groningen_analytics::modules::helpers::logging::setup_logging;
use karting_groningen_analytics::modules::models::general::establish_connection;

#[tokio::main]
async fn main() {
    dotenv().ok();
    setup_logging().expect("failed to setup logging");


    // get all the heats stored in the file
    let file_url = "./src/heats.txt";
    let heat_list: Vec<String> = match HeatsHelper::load_heat_ids_from_file(file_url) {
        Ok(heats) => heats,
        Error::FileDoesNotExistError {} => {
            error!(target:"load_files-From_heat", "File does not exist: {}", path);
            return;
        }
        Error::PermissionDeniedError {} => {
            error!(target:"load_files-From_heat", "Permission denied: {}", path);
            return;
        }
        _ => unreachable!("Unexpected error: {:?}", error)
    };

    // get the info from the heats and save into database
    let connection = &mut establish_connection();
    for heat_id in heat_list {
        match get_heat_from_api(heat_id).await {
            Ok(heat) => {
                match save_heat(connection, heat) {
                    Ok(_) => {
                        info!(target:"load_heats_from_file", "saved heat: {}", heat_id);
                    },
                    Error::AlreadyExistsError{ .. } => {
                        info!(target:"load_heats_from_file", "heat already exists: {}", heat_id);
                    }
                    Error::InvalidNameError{ .. } => {
                        warn!(target:"load_heats_from_file", "invalid driver names in heat {}", heat_id);
                    }
                    _ => {unreachable!()}
                    }
            }
            Err(err) => {
                error!(target:"load_heats_from_file", "failed loading heat from api. (heat_id: {})", err);
            }
        };

    }
}
