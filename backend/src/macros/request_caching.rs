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

pub(crate) use read_cache_request;
pub(crate) use cache_response;
