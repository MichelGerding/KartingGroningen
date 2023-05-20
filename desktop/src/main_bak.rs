use std::fs::File;
use std::thread::sleep;
use std::time::Duration;
use sysinfo::{ProcessExt, System, SystemExt};

use simetry::iracing::{Client as iClient, DiskClient, Value};


enum Game {
    Iracing,
    AssettoCorsa,
    AssettoCorsaCompetitione,
}
impl Game {
    pub fn from_string(game: &str) -> Option<Game> {
        match game {
            "iRacing" => Some(Game::Iracing),
            "ac" => Some(Game::AssettoCorsa),
            "acc" => Some(Game::AssettoCorsaCompetitione),
            _ => {None}
        }
    }
}


#[tokio::main]
async fn main() {
    let mut client = DiskClient::open(r"C:\Users\miche\Documents\iRacing\telemetry\rt2000_mtwashington climb 2023-04-27 20-27-02.ibt").unwrap();

    if !client.variables().contains_key("Lat") || !client.variables().contains_key("Lon") {
        return;
    }

    let mut positions:Vec<(Value, Value)> = Vec::new();
    while let Some(sim_state) = client.next_sim_state() {
        if !sim_state.variables().contains_key("Lat") || !sim_state.variables().contains_key("Lon") {
            continue;
        }
        let lat: f64 = sim_state.read_name("Lat").unwrap();
        let lon: f64 = sim_state.read_name("Lon").unwrap();
        // positions.push((lat, lon));
        dbg!((lat, lon));

        sleep(Duration::from_secs_f64(1.0/60.0))
    }

    // export into csv
    // let path = "positions.txt";
    // let mut output = File::create(path).unwrap();
    // write!(output, "lat, lon").expect("failed to write");
    // for pos in positions {
    //     write!(output, "{}, {}", pos.0, pos.1).expect("failed to write");
    // }

    // for (key, val) in client.variables() {
    //     println!("{}: {:?}", key, val)
    // }

    // return;


    // return;
    let s = System::new_all();
    let games = [
        "iRacing",
    ];


    let mut running_game: Option<Game> = None;

    while running_game.is_none() {
        // check if we can find a running game
        for game in games {
            let mut processes = s.processes_by_name(game);

            let sim = &processes.find(|p| {
                p.name().contains("iRacingSim")
            });

            match sim {
                None => {
                    println!("sim not running");
                    continue;
                },
                Some(_) => {
                    running_game = Game::from_string(game);
                    break;
                }
            }

        }
        sleep(Duration::from_secs(1));
    }

    match running_game.unwrap() {
        Game::Iracing => {
            let mut client = iClient::connect().await.unwrap();
            println!("Connected!");
            while let Some(_sim_state) = client.next_sim_state().await {
                // println!("{_sim_state:#?}");
                // println!("{:#?}", _sim_state.);
                // let header =
                // let lat: Option<Value> = _sim_state.read(_sim_state.variables().get("Lat").unwrap());
                break;
            }
        }
        Game::AssettoCorsa => {}
        Game::AssettoCorsaCompetitione => {}
    }
}