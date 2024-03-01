use core::panic;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Error, Read},
};

use serde::Deserialize;

pub struct Utils {}

pub enum FileReadingReturnTypes {
    ReturnEntireFile(String),
    ReturnBufferHandle(BufReader<File>),
}

pub enum JSONErrorTypes {
    JSONError(serde_json::Error),
    Error(Error),
}

impl Utils {
    pub fn read_file(
        path_to_file: &str,
        panic_if_absent: bool,
        stream: bool,
    ) -> Result<FileReadingReturnTypes, Error> {
        // (try to) read the file first

        let file_handle = File::open(path_to_file)?;

        if file_handle.is_err() && panic_if_absent == true {
            panic!("Some error while trying to read the file");
        } else {
            // if stream == true {
            //     // read it using a bufreader and return the handle orrr
            //     // return the whole file after reading it into memory
            // } else {
            // }

            match stream {
                true => {}
                false => String::from_utf8_lossy(file_handle),
            }
        }
    }

    pub fn write_to_file(path_to_file: &str, panic_if_absent: bool) {}

    pub fn parse_to_json<'a, json_blue_print>(path_to_file: &str) -> Result<_, JSONErrorTypes>
    where
        json_blue_print: Deserialize<'a>,
    {
        let contents_to_string = Utils::read_file(path_to_file, true, false)?;

        let mut contents_to_json = match contents_to_string {
            FileReadingReturnTypes::ReturnBufferHandle(buffer) => {
                let mut m: String;
                for line in buffer.lines() {
                    m.push_str(&line.unwrap());
                }

                serde_json::from_str(&m)
            }
            FileReadingReturnTypes::ReturnEntireFile(file_as_string) => {
                serde_json::from_str(&file_as_string)
            }
        };

        contents_to_json
    }
}
