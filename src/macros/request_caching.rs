/// check if a request is in the cache, if it is, return it.
/// else follow the normal flow
///
/// does nothing when debug enabled
macro_rules! read_cache_request {
    ( $origin:expr ) => {
        if !cfg!(debug_assertions) {
            let uri = $origin.path().to_string();
            let r_con = &mut Redis::connect();

            if Redis::has_data::<String>(r_con,uri.clone()).unwrap() {
                let data = Redis::get_data::<String, String>(r_con,uri.clone()).unwrap();
                let api_driver = serde_json::from_str(&data).unwrap();
                return Ok(api_driver);
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
            let r_con = &mut Redis::connect();
            let response_str = serde_json::to_string(&$data).unwrap();

            Redis::set_data::<String, String>(r_con,uri, response_str.clone());
        }

        return Ok($data);
    }
}

pub(crate) use read_cache_request;
pub(crate) use cache_response;