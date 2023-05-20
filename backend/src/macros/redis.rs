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



pub(crate) use delete_keys;
