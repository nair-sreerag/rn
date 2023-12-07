use serde::{Deserialize, Serialize};
use std::fs;
pub struct Utils {}

impl Utils {
    pub fn read_file(path: &str) -> String {
        let contents = fs::read_to_string(path);

        return contents.unwrap();
    }

    pub fn read_dir(path: &str) {}

    pub fn convert_to_json<'a, T>(json_string: &'a str) -> T
    where
        T: Deserialize<'a> + Serialize,
    {
        let parsed_json: T = serde_json::from_str(&json_string).unwrap();

        parsed_json
    }
}
