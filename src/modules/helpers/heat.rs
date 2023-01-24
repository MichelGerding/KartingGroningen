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
    pub fn load_heat_ids_from_file(filename: &str) -> Vec<String> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let path = Path::new(filename);

        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut heat_list: Vec<String> = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap(); // Ignore errors.
            heat_list.push(line);
        }
        heat_list
    }
}
