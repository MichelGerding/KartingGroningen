use std::thread;
// rocket imports
use rocket::get;
use rocket::http::Status;
use rocket::http::uri::Origin;
use rocket_dyn_templates::Template;

//helper imports
use crate::modules::helpers::math::Math;
use crate::{
    AllData, ChartData, ChartDataDataSetData, ChartDataDataset, TableData, TemplateDataHeat,
};
// database imports
use crate::modules::models::general::establish_connection;
use crate::modules::models::heat::{Heat, HeatDriverInfo};
use crate::modules::models::lap::Lap;
use crate::modules::redis::Redis;

#[get("/all")]
pub fn list_all(origin: &Origin) -> Template {

    // check the cache
    let r_conn = &mut Redis::connect();
    let uri = origin.path().to_string();

    let all_data;
    if Redis::has_data(r_conn, uri.clone()).unwrap() {
        all_data = serde_json::from_str(&Redis::get_data::<String, String>(r_conn, uri.clone()).unwrap()).unwrap()
    } else {
        let connection = &mut establish_connection();
        let heats = Heat::get_all_with_stats(connection);

        all_data = AllData {
            data_type: "heats".to_string(),
            table_data: TableData {
                headers: vec![
                    "Id".to_string(),
                    "Type".to_string(),
                    "Laps".to_string(),
                    "Drivers".to_string(),
                    "Date".to_string(),
                    "".to_string(),
                ],
                rows: heats
                    .iter()
                    .map(|heat| {
                        vec![
                            heat.heat_id.to_string(),
                            heat.heat_type.to_string(),
                            heat.amount_of_laps.to_string(),
                            heat.amount_of_drivers.to_string(),
                            heat.start_time.to_string(),
                        ]
                    })
                    .collect(),
            },
        };

        let ad = all_data.clone();
        thread::spawn(move || {
            let r_conn = &mut Redis::connect();

            let json = serde_json::to_string(&ad).unwrap();
            Redis::set_data::<String, String>(r_conn, uri, json);
        });
    }
    Template::render(
        "all",
        all_data
    )
}

#[get("/<heat_id_in>")]
pub fn single(heat_id_in: String, origin: &Origin) -> Result<Template, Status> {
    // let sanitized = sanitize_name(&heat_id_in);
    // if sanitized != heat_id_in {
    //     return Err(Status::BadRequest)
    // }

    let r_conn = &mut Redis::connect();
    let uri = origin.path().to_string();

    let all_data;
    if Redis::has_data(r_conn, uri.clone()).unwrap() {
        all_data = serde_json::from_str(&Redis::get_data::<String, String>(r_conn, uri.clone()).unwrap()).unwrap()
    } else {


        let conn = &mut establish_connection();

        // get all data needed
        let heat = Heat::get_by_id(conn, &heat_id_in);
        let heat_info = heat.unwrap().get_full_info(conn);

        let mut table_rows: Vec<Vec<String>> = Vec::new();
        let mut datasets: Vec<ChartDataDataset> = Vec::new();

        // generate the data to show in the charts and the table
        for driver in &heat_info.drivers {
            table_rows.push(generate_table_row(driver));
            datasets.push(generate_chart_data(driver));
        }


        all_data = TemplateDataHeat {
            heat_id: heat_info.heat_id,
            heat_type: heat_info.heat_type,
            start_date: heat_info.start_time,
            chart_data: ChartData {
                labels: datasets.iter().map(|x| x.label.clone()).collect(),
                datasets,
            },
            table_data: TableData {
                headers: vec![
                    "Driver".to_string(),
                    "Kart".to_string(),
                    "Median Laptime".to_string(),
                    "Average Laptime".to_string(),
                    "Fastest Laptime".to_string(),
                    "Fastest Lap".to_string(),
                    "Total Laps".to_string(),
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

    Ok(Template::render(
        "heat",
        all_data
    ))
}

fn generate_table_row(driver: &HeatDriverInfo) -> Vec<String> {
    let stats = Lap::get_stats_of_laps(&driver.laps);
    let fastest_lap = driver
        .laps
        .iter()
        .find(|e| e.lap_time == stats.fastest_lap_time)
        .unwrap();

    vec![
        format!("<a href=\"/drivers/{}\"> {}</a>", driver.name, driver.name),
        format!(
            "<a href=\"/karts/{}\"> {}</a>",
            driver.kart.number, driver.kart.number
        ),
        Math::round_float_to_n_decimals(stats.median_lap_time, 2).to_string(),
        Math::round_float_to_n_decimals(stats.avg_lap_time, 2).to_string(),
        Math::round_float_to_n_decimals(fastest_lap.lap_time, 2).to_string(),
        fastest_lap.lap_in_heat.to_string(),
        driver.laps.len().to_string(),
    ]
}

fn generate_chart_data(driver: &HeatDriverInfo) -> ChartDataDataset {
    ChartDataDataset {
        label: driver.name.clone(),
        data: driver
            .laps
            .iter()
            .map(|y| ChartDataDataSetData {
                date: None,
                driver: None,
                lap_time: y.lap_time,
            })
            .collect::<Vec<ChartDataDataSetData>>(),
    }
}
