
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



pub(crate) use clear_cache;
pub(crate) use delete_keys;
