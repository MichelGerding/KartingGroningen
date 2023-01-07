pub struct HeatsHelper {}

impl HeatsHelper {
    pub fn load_heat_ids_from_file(filename: &str) -> Vec<String> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut heat_list: Vec<String> = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap(); // Ignore errors.
            heat_list.push(line);
        }

        heat_list
    }
}