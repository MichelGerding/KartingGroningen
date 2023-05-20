use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::thread::sleep;
use std::time::Duration;
use bincode::{Decode, Encode};
use sysinfo::{ProcessExt, System, SystemExt};

use simetry::iracing::{Client as iClient, DiskClient, Value};


enum Game {
    Iracing,
    AssettoCorsa,
    AssettoCorsaCompetitione,
}

struct DataFrames {
    frames: Vec<DataFrame>
}

impl Debug for DataFrames {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("frames: {}", self.frames.len()))
    }
}

#[derive(Debug)]
struct SessionInfo {
    pub trackname: String,
    pub tick_rate: u8,

    pub data: DataFrames,
}

#[derive(Encode, Decode, Debug)]
struct DataFrame {
    // genral info
    pub lap: u16,

    // position
    pub lat: f32,
    pub lon: f32,

    // inputs
    pub throttle: u8,
    pub brake: u8,
    pub steering: f32,

    // rotation
    pub gear: u8,
    pub engine_rpm: u16,
    pub speed: u16,
}

impl Game {
    pub fn from_string(game: &str) -> Option<Game> {
        match game {
            "iRacing" => Some(Game::Iracing),
            "ac" => Some(Game::AssettoCorsa),
            "acc" => Some(Game::AssettoCorsaCompetitione),
            _ => { None }
        }
    }
}


#[tokio::main]
async fn main() {
    let file = r"C:\Users\miche\Documents\iRacing\telemetry\rt2000_mtwashington climb 2023-04-27 20-27-02.ibt";
    let mut client = DiskClient::open(file).unwrap();

    let mut session = SessionInfo {
        trackname: "".to_string(),
        tick_rate: client.header().tick_rate as u8,
        data: DataFrames {
            frames: vec![],
        },
    };

    // load the file from disk and store into the session info objects
    while let Some(sim_state) = client.next_sim_state() {
        let lat: f32 = sim_state.read_name::<f64>("Lat").unwrap() as f32;
        let lon: f32 = sim_state.read_name::<f64>("Lon").unwrap() as f32;

        let throttle: u8 = (sim_state.read_name::<f64>("Throttle").unwrap() * 100.0) as u8;
        let brake: u8 = (sim_state.read_name::<f64>("Brake").unwrap() * 100.0) as u8;
        let steering: f32 = sim_state.read_name("SteeringWheelAngle").unwrap();
        let speed: u16 = (sim_state.read_name::<f64>("Speed").unwrap() * 3.6) as u16;
        let gear: u8 = sim_state.read_name::<i32>("Gear").unwrap() as u8;
        let engine_rpm: u16 = sim_state.read_name::<f64>("RPM").unwrap() as u16;
        let lap: u16 = sim_state.read_name::<i32>("Lap").unwrap() as u16;

        let frame = DataFrame {
            lap,
            lat,
            lon,
            throttle,
            brake,
            steering,
            gear,
            engine_rpm,
            speed,
        };
        session.data.frames.push(frame);
    }

    dbg!(session);
    return;

    // load the sim
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
                }
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