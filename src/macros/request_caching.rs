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
            let uri = $origin.path().to_string();
            match &mut Redis::connect() {
                Ok(r_conn) => {
                    let response_str = serde_json::to_string(&$data).unwrap();
                    let _ = Redis::set_data::<String, String>(r_conn,uri, response_str.clone());
                },
                Err(error) => {
                    error!(target:"routes/heat:list_all", "Error connecting to redis: {}", error);
                    return Err(Status::InternalServerError);
                }
            }
        }

        return Ok($data)
    }
}

macro_rules! cache_template_response {
    ($template:expr, $uri:expr, $logging_target:expr, $data_type:ty, $not_cached:expr) => {
        match &mut Redis::connect() {
            Ok(r_conn) => {
                let has_data = match Redis::has_data(r_conn, $uri.clone()) {
                    Ok(b) => b,
                    Err(error) => {
                        error!(target: $logging_target, "Error checking redis for data: {}", error);
                        return Err(Status::InternalServerError);
                    }
                };

                if has_data {
                    match Redis::get_data::<String, String>(r_conn, $uri.clone()) {
                        Ok(d) => {
                            Ok(Template::render(
                                $template,
                                serde_json::from_str::<$data_type>(&d).unwrap()
                            ))
                        }
                        Err(error) => {
                            error!(target: $logging_target, "Error getting data from redis: {}", error);
                            return Err(Status::InternalServerError);
                        }
                    }
                } else {
                    let data = $not_cached();
                    match data {
                        Ok(d) => {
                             cache_data_to_url!(d, $uri, $logging_target);
                            Ok(Template::render($template, d))
                        }
                        Err(error) => {
                            error!(target: $logging_target, "Error loading page data: {}", error);
                            return Err(Status::InternalServerError);
                        }
                    }
                }
            }
            Err(error) => {
                error!(target: $logging_target, "Error connecting to redis: {}", error);
                return Err(Status::InternalServerError);
            }
        }
    }
}


pub(crate) use read_cache_request;
pub(crate) use cache_response;
pub(crate) use cache_template_response;