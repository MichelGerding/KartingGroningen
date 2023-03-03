use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::thread;

// Datatypes
use chrono::NaiveDate;
use diesel::PgConnection;
use log::error;
// Rocket imports
use rocket::get;
use rocket::http::Status;
use rocket::http::uri::Origin;
use rocket_dyn_templates::Template;
// Utility imports
use serde::{Deserialize, Serialize};

// Templating imports
use crate::{AllDataWithCharts, ChartData, ChartDataDataset, ChartDataDataSetData, TableData};
use crate::macros::database_error_handeler::db_handle_get_error_http;
use crate::macros::redis::{cache_data_to_url, redis_handle_set_error_no_return};
use crate::macros::request_caching::cache_template_response;
// Database imports
use crate::modules::models::driver::Driver;
use crate::modules::models::general::establish_connection;
use crate::modules::models::heat::Heat;
use crate::modules::models::kart::{Kart, KartStatsPerDay};
use crate::modules::models::lap::Lap;
use crate::modules::redis::Redis;

#[get("/all")]
pub fn list_all(origin: &Origin) -> Result<Template, Status> {
    let uri = origin.path().to_string();
    cache_template_response!(
        "all",
        uri,
        "routes/kart:list_all",
        AllDataWithCharts,
        || -> Result<AllDataWithCharts, Status> {
            let connection: &mut PgConnection = &mut establish_connection();
            let karts_stats = Kart::get_all_with_stats(connection, String::from("number"), String::from("asc"));

            let table_rows = karts_stats
                .iter()
                .map(|stats| {
                    let is_child_kart = if stats.is_child_kart {
                        "Child Kart"
                    } else {
                        "Adult Kart"
                    };
                    if stats.lap_count == 0 {
                        return vec![
                            stats.number.to_string(),
                            is_child_kart.to_string(),
                            "0".to_string(),
                            "0".to_string(),
                        ];
                    }

                    // return the data to the kart as strings so we can show it in a table
                    vec![
                        stats.number.to_string(),
                        is_child_kart.to_string(),
                        stats.lap_count.to_string(),
                        stats.driver_count.to_string(),
                    ]
                })
                .collect(); // get the stats of all karts per day

            let kart_stats: HashMap<Kart, Vec<KartStatsPerDay>> =
                Kart::get_stats_per_day_from_db(connection);

            let datasets: Vec<ChartDataDataset> = kart_stats
                .iter()
                .map(|(kart, stats)| {
                    let min_per_day: HashMap<NaiveDate, f64> = stats
                        .iter()
                        .map(|stat| (stat.start_date.date(), stat.min_laptime))
                        .collect();

                    ChartDataDataset {
                        label: kart.number.to_string(),
                        data: list_to_chart_data_set_without_driver(min_per_day.iter()),
                    }
                })
                .collect();

            Ok(AllDataWithCharts {
                data_type: "karts".to_string(),
                chart_data: ChartData {
                    labels: datasets.iter().map(|x| x.label.to_owned()).collect(),
                    datasets,
                },
                table_data: TableData {
                    headers: vec![
                        "Number".to_string(),
                        "Child Kart".to_string(),
                        "Total Laps".to_string(),
                        "Total Drivers".to_string(),
                        "".to_string(),
                    ],
                    rows: table_rows,
                },
            })
    })
}

#[get("/<kart_number>")]
pub fn single(kart_number: i32, origin: &Origin) -> Result<Template, Status> {
    let uri = origin.path().to_string();
    cache_template_response!(
        "kart",
        uri,
        "routes/kart:single",
        TemplateDataKart,
        || -> Result<TemplateDataKart, Status> {
            let connection = &mut establish_connection();
            let kart = match Kart::get_by_number(connection, kart_number) {
                Ok(kart) => kart,
                Err(_) => {
                    return Err(Status::NotFound);
                }
            };

            // get all driven laps drivers and heats of the kart
            let laps = db_handle_get_error_http!(Lap::from_kart(connection, &kart), "/routes/kart:single", format!("laps from kart {}", kart_number));

            let heats = db_handle_get_error_http!(Heat::from_laps(connection, &laps), "routes/kart:single", format!("heats from kart {}", kart_number));
            let drivers =  db_handle_get_error_http!(Driver::from_laps(connection, &laps), "routes/kart:single", format!("drivers from kart {}", kart_number));

            // get the daily stats of the kart. these are the average, median and minimum laptime.
            let avg_per_day = db_handle_get_error_http!(kart.get_average_laptime_per_day(connection), "routes/kart:single", " average laptime per day");
            let min_per_day = db_handle_get_error_http!(kart.get_minimum_laptime_per_day(connection), "routes/kart:single", " minimum laptime per day");
            let med_per_day = db_handle_get_error_http!(kart.get_median_laptime_per_day(connection), "routes/kart:single", " median laptime per day");

            // get all datasets for the charts on the page
            let datasets: Vec<ChartDataDataset> = vec![
                ChartDataDataset {
                    label: "Average".to_string(),
                    data: list_to_chart_data_set_without_driver(avg_per_day.iter()),
                },
                ChartDataDataset {
                    label: "Minimum".to_string(),
                    data: list_to_chart_data_set_without_driver(min_per_day.iter()),
                },
                ChartDataDataset {
                    label: "Median".to_string(),
                    data: list_to_chart_data_set_without_driver(med_per_day.iter()),
                },
                ChartDataDataset {
                    label: "All Laps".to_string(),
                    data: list_to_chart_data_set(&laps, &heats, &drivers),
                },
                ChartDataDataset {
                    label: "All Laps (Normalized)".to_string(),
                    data: list_to_chart_data_set(&Lap::filter_outliers(&laps), &heats, &drivers),
                },
            ];

            Ok(TemplateDataKart {
                number: kart.number,
                is_child_kart: kart.is_child_kart,
                total_laps: laps.len() as i32,
                total_drivers: drivers.len() as i32,
                chart_data: ChartData {
                    labels: datasets.iter().map(|x| x.label.to_owned()).collect(),
                    datasets,
                },
                table_data: TableData {
                    headers: vec![
                        "Driver".to_string(),
                        "Average Lap Time".to_string(),
                        "Best Lap Time".to_string(),
                        "Total Laps".to_string()],
                    rows: drivers.iter().map(|driver| {
                        let driver_stats = driver.get_stats_for_laps(connection, &laps);

                        vec![
                            format!("<a href=\"/drivers/{}\"> {}</a>", driver.name, driver.name),
                            driver_stats.avg_lap_time.to_string(),
                            driver_stats.fastest_lap.lap_time.to_string(),
                            driver_stats.total_laps.to_string(),
                        ]
                    }).collect()
                }
            })

      })
}

// CODE DEDUPLICATION FUNCTIONS
fn list_to_chart_data_set_without_driver(laps: Iter<NaiveDate, f64>) -> Vec<ChartDataDataSetData> {
    laps.map(|(date, laptime)| ChartDataDataSetData {
        date: Some(date.to_owned()),
        driver: None,
        lap_time: laptime.to_owned(),
    })
        .collect()
}

fn list_to_chart_data_set(
    laps: &[Lap],
    heats: &[Heat],
    drivers: &[Driver],
) -> Vec<ChartDataDataSetData> {
    let drivers_hash_map = drivers
        .iter()
        .map(|driver| (driver.id, driver.to_owned()))
        .collect::<HashMap<i32, Driver>>();
    let heats_hash_map = heats
        .iter()
        .map(|heat| (heat.id, heat))
        .collect::<HashMap<i32, &Heat>>();

    laps.iter()
        .map(|x| ChartDataDataSetData {
            date: Some(heats_hash_map.get(&x.heat).unwrap().start_date.date()),
            driver:
            Some(drivers_hash_map
                .get(&x.driver)
                .unwrap()
                .clone()),
            lap_time: x.lap_time,
        })
        .collect()
}


#[derive(Serialize, Deserialize, Clone)]
struct TemplateDataKart {
    pub number: i32,
    pub is_child_kart: bool,
    pub total_laps: i32,
    pub total_drivers: i32,
    pub table_data: TableData,
    pub chart_data: ChartData,
}