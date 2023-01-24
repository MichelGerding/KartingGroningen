// Datatypes
use chrono::NaiveDate;
use diesel::PgConnection;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::thread;
// Rocket imports
use rocket::get;
use rocket::http::Status;
use rocket::http::uri::Origin;
use rocket_dyn_templates::{Template};
// Utility imports
use serde::{Deserialize, Serialize};
// Templating imports
use crate::{AllDataWithCharts, ChartData, ChartDataDataSetData, ChartDataDataset, TableData};
// Database imports
use crate::modules::models::driver::Driver;
use crate::modules::models::general::establish_connection;
use crate::modules::models::heat::Heat;
use crate::modules::models::kart::{Kart, KartStatsPerDay};
use crate::modules::models::lap::Lap;
use crate::modules::redis::Redis;

#[get("/all")]
pub fn list_all(origin: &Origin) -> Template {

    //TODO:: optimize to no longer need all laps

    // get all karts and all laps
    // we get all laps because we want to get some basic information of the kart.
    // because we show all karts we need to also get all the laps because all laps have a kart


    let r_conn = &mut Redis::connect();
    let uri = origin.path().to_string();

    let all_data;
    if Redis::has_data(r_conn, uri.clone()).unwrap() && false {
        all_data = serde_json::from_str(&Redis::get_data::<String, String>(r_conn, uri.clone()).unwrap()).unwrap()
    } else {
        let connection: &mut PgConnection = &mut establish_connection();
        let karts_stats = Kart::get_all_with_stats(connection);

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

        dbg!(&kart_stats.len());

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

        all_data = AllDataWithCharts {
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
        };

        let ad = all_data.clone();
        thread::spawn(move || {
            let r_conn = &mut Redis::connect();

            let json = serde_json::to_string(&ad).unwrap();
            Redis::set_data::<String, String>(r_conn, uri, json);
        });
    }
    // return the template and the data needed to render it
    Template::render(
        "all",
        all_data,
    )
}

#[get("/<kart_number>")]
pub fn single(kart_number: i32, origin: &Origin) -> Result<Template, Status> {

    let r_conn = &mut Redis::connect();
    let uri = origin.path().to_string();

    let all_data;
    if Redis::has_data(r_conn, uri.clone()).unwrap() {
        all_data = serde_json::from_str(&Redis::get_data::<String, String>(r_conn, uri.clone()).unwrap()).unwrap()
    } else {
        let connection = &mut establish_connection();
        let kart = match Kart::get_by_number(connection, kart_number) {
            Ok(kart) => kart,
            Err(_) => {
                return Err(Status::NotFound);
            }
        };

        // get all driven laps drivers and heats of the kart
        let laps = Lap::from_kart(connection, &kart);
        let drivers = Driver::from_laps(connection, &laps);
        let heats = Heat::from_laps(connection, &laps);

        // get the daily stats of the kart. these are the average, median and minimum laptime.
        let avg_per_day = kart.get_average_laptime_per_day(connection);
        let min_per_day = kart.get_minimum_laptime_per_day(connection);
        let med_per_day = kart.get_median_laptime_per_day(connection);

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

        all_data = TemplateDataKart {
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
        };

        let ad = all_data.clone();
        thread::spawn(move || {
            let r_conn = &mut Redis::connect();

            let json = serde_json::to_string(&ad).unwrap();
            Redis::set_data::<String, String>(r_conn, uri, json);
        });
    }

    Ok(Template::render(
        "kart",
        all_data
    ))
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


#[derive(Serialize, Deserialize,  Clone)]
struct TemplateDataKart {
    pub number: i32,
    pub is_child_kart: bool,
    pub total_laps: i32,
    pub total_drivers: i32,
    pub table_data: TableData,
    pub chart_data: ChartData,
}