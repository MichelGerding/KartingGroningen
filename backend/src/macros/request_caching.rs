/// check if a request is in the cache, if it is, return it.
/// else follow the normal flow
///
/// does nothing when debug enabled
macro_rules! read_cache_request {
    ( $origin:expr ) => {
        if !cfg!(debug_assertions) {
            let uri = $origin.path().to_string();
            match &mut Redis::connect() {
                Ok(r_conn) => {
                    if Redis::has_data::<String>(r_conn,uri.clone()).unwrap() {
                        let data = Redis::get_data::<String, String>(r_conn,uri.clone()).unwrap();
                        let api_driver = serde_json::from_str(&data).unwrap();
                        return Ok(api_driver);
                    }
                },
                Err(error) => {
                    error!(target:"routes/heat:list_all", "Error connecting to redis: {}", error);
                    return Err(Status::InternalServerError);
                }
            }
        }
    }
}

/// add the response to the request to the cache and then return it.
///
/// if debug is enabled we wont add to cache.
macro_rules! cache_response {
    ( $origin:expr, $data:expr ) => {
        if !cfg!(debug_assertions) {

            let response_str = serde_json::to_string(&$data).unwrap();
            let uri = $origin.path().to_string();
            let _ = task_queue::add_task(Box::new(move || {
                match &mut Redis::connect() {
                    Ok(r_conn) => {
                        let _ = Redis::set_data::<String, String>(r_conn, uri.clone(), response_str.clone());
                    },
                    Err(error) => {
                        error!(target:"routes/heat:list_all", "Error connecting to redis: {}", error);
                    }
                }
            }));
        }

        return Ok($data)
    }
}


macro_rules! cache_generation {
    ($origin:expr, $dt:ty, $gen:expr) =>{
        // check if we build in debug mode
            let uri = $origin.path().to_string();

            match &mut Redis::connect() {
                Ok(redis_connection) => {
                    // connected to redis.
                    // check if the current url has data
                    if Redis::has_data::<String>(redis_connection, uri.clone()).unwrap() {
                        // the data already exists in chache
                        match Redis::get_data::<String, String>(redis_connection, uri.clone()) {
                            Ok(data) => {
                                // let parsed_data: Result<KartStats, Status> = serde_json::from_str(&data).unwrap();
                                return Ok(data);
                            },
                            Err(error) => {
                                error!(target:"redis::get_data", "Error getting data from redis: {}", error);
                                return Err(Status::InternalServerError);
                            }
                        };
                    }


                    // generate the data and store it in database
                    match $gen() {
                        Ok(data) => {
                            return Ok(data);
                        }
                        Err(error) => {
                            error!(target:"redis::get_data", "Error getting data from redis: {}", error);
                            return Err(Status::InternalServerError);
                        }
                    };
                }
                Err(error) => {
                    error!(target:"redis::connect", "Error connectiong to redis: {}", error);
                    return Err(Status::InternalServerError);
                }
            }
        }
}



pub(crate) use cache_generation;
pub(crate) use cache_response;
pub(crate) use read_cache_request;
