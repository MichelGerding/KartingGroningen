macro_rules! redis_handle_set_error_no_return {
    ( $data:expr, $target:expr, $type_str:expr) => {
        match $data {
            Ok(e) => e,
            Err(error) => {
                error!(target:$target, "Error setting {}. (error: {})", $type_str, error);
                "".to_string()
            }
        }
    }
}

macro_rules! cache_data_to_url {
    ( $data:expr, $url:expr, $target:expr) => {
        let ad = $data.clone();
        thread::spawn(move || {
            let r_conn_m = &mut Redis::connect();
            if r_conn_m.is_err() {
                error!("Error connecting to redis");
                return Err(Status::InternalServerError);
            }
            let r_conn = &mut r_conn_m.as_mut().unwrap();

            let json = serde_json::to_string(&ad).unwrap();
            redis_handle_set_error_no_return!(Redis::set_data::<String, String>(r_conn, $url, json), "routes/driver:list_all", "Error while setting data in redis: {}");
            Ok(())
        });
    }
}

macro_rules! clear_cache {
    ($target:expr) => {
        match &mut Redis::connect() {
            Ok(r_conn) => {
                $target.clear_cache(r_conn);
            }
            Err(error) => {
                error!(target:"models/driver::new", "Error connecting to redis: {}", error);
            }
        }
    }
}

macro_rules! delete_keys {
    ($conn:expr, $keys:expr, $target:expr) => {
        for key in $keys {
            match Redis::delete($conn, &key) {
                Ok(_) => {}
                Err(error) => {
                    error!(target:$target, "Error while deleting key: {}", error);
                }
            };
        }
    }
}



pub(crate) use redis_handle_set_error_no_return;
pub(crate) use clear_cache;
pub(crate) use cache_data_to_url;
pub(crate) use delete_keys;
