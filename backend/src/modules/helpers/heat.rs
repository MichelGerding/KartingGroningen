use crate::errors::{CustomResult, Error};
use log::warn;
use std::io::ErrorKind::PermissionDenied;
use std::path::Path;

pub struct HeatsHelper {}

impl HeatsHelper {
    /// # load heat_id's from a file
    /// load all heat_id's stored in a file
    ///
    /// ## Arguments
    /// * `filename` - The path to the file to load
    ///
    /// ## Returns
    /// * 'Vec<String>' - A vector of heat_id's
    pub fn load_heat_ids_from_file(filename: &str) -> CustomResult<Vec<String>> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let path = Path::new(filename);

        let file = match File::open(path) {
            Ok(file) => file,

            Err(error) => {
                return match error.kind() {
                    PermissionDenied => Err(Error::PermissionDeniedError {}),
                    _ => Err(Error::FileDoesNotExistError {}),
                }
            }
        };
        let reader = BufReader::new(file);

        let mut heat_list: Vec<String> = Vec::new();
        for (i, line) in reader.lines().enumerate() {
            match line {
                Ok(line) => heat_list.push(line),
                Err(error) => {
                    warn!(target:"helpers/heat:load_heat_ids_from_file", "Error reading line: {}. (error: {})", i, error);
                }
            };
        }

        Ok(heat_list)
    }
}
