use core::panic;
use std::{
    fs::{self, File},
    io::{Error, Read},
};

pub struct Utils {}

pub enum ReadFileReturnTypes {}

impl Utils {
    pub fn read_file(
        path_to_file: &str,
        panic_if_absent: bool,
        stream: bool,
    ) -> Result<Vec<u8>, Error> {
        // (try to) read the file first

        let file_handle = File::open(path_to_file);

        if file_handle.is_err() && panic_if_absent == true {
            panic!("Some error while trying to read the file");
        } else {
            if stream == true {
                // read it using a bufreader and return the handle orrr
                // return the whole file after reading it into memory
            } else {
            }
        }
    }

    pub fn write_to_file(path_to_file: &str, panic_if_absent: bool) {}

    pub fn parse_to_json<blue_print>(path_to_file: &str) -> Result<_, Error> {
        let contents_to_string = Utils::read_file(path_to_file, true);

        let contents_to_json = serde_json::from_str(contents_to_string);
    }
}
