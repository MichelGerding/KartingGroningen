macro_rules! db_handle_get_error_http {
    ( $data:expr, $target:expr, $type_str:expr) => {
        match $data {
            Ok(e) => e,
            Err(diesel::result::Error::NotFound) => {
                return Err(Status::NotFound);
            }
            Err(error) => {
                error!(target:$target, "Error getting {}. (error: {})", $type_str, error);
                return Err(Status::InternalServerError);
            }
        }
    }
}
macro_rules! db_handle_get_error {
    ( $data:expr, $target:expr, $type_str:expr) => {
        match $data {
            Ok(e) => e,
            Err(diesel::result::Error::NotFound) => {
                return Err(diesel::result::Error::NotFound);
            }
            Err(error) => {
                error!(target:$target, "Error getting {}. (error: {})", $type_str, error);
                return Err(error);
            }
        }
    }
}

pub(crate) use db_handle_get_error_http;
pub(crate) use db_handle_get_error;

