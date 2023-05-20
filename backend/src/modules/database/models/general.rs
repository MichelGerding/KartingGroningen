use dotenvy::dotenv;
use std::env;
use std::sync::{Mutex};
use deadpool_postgres::{Config, CreatePoolError, Pool, Runtime};
use tokio_postgres::{NoTls};


pub fn get_config() -> Config {
    dotenv().ok();
    unsafe {
        if CONNECTION_CONFIG.is_none() {
            let mut cfg = Config::new();
            cfg.user = Some(env::var("PG__USER").expect("PG__USER must be set"));
            cfg.password = Some(env::var("PG__PASSWORD").expect("PG__PASSWORD must be set"));
            cfg.host = Some(env::var("PG__HOST").expect("PG__HOST must be set"));
            cfg.port = Some(env::var("PG__PORT").expect("PG__PORT must be set").parse::<u16>().unwrap());
            cfg.dbname = Some(env::var("PG__DBNAME").expect("PG__DBNAME must be set"));

            CONNECTION_CONFIG = Some(Mutex::new(cfg));
        }
    }

    unsafe { CONNECTION_CONFIG.as_ref().unwrap().lock().unwrap().clone() }
}

pub async fn create_pool() -> Result<Pool, CreatePoolError> {
    let cfg = get_config();
    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
}


// create a global connection pool
pub static mut CONNECTION_POOL: Option<Mutex<Pool>> = None;
pub static mut CONNECTION_CONFIG: Option<Mutex<Config>> = None;

pub async fn setup_pool() {
    unsafe {
        CONNECTION_POOL = Some(Mutex::new(create_pool().await.unwrap()));
    }
}

pub fn get_pool() -> Pool {
    // return the pool and not the mutex.
    unsafe { CONNECTION_POOL.as_ref().unwrap().lock().unwrap().clone() }
}