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

#[derive(Debug)]
pub enum JSONErrorTypes {
    JSONError(serde_json::Error),
    Error(Error),
}

impl Utils {
    pub fn read_file(
        path_to_file: &str,
        panic_if_absent: bool,
        streamify: bool,
    ) -> Result<FileReadingReturnTypes, Error> {
        // (try to) read the file first

        let file_open_try = File::open(path_to_file);

        match file_open_try {
            Ok(mut file_handle) => match streamify {
                true => Ok(FileReadingReturnTypes::ReturnBufferHandle(BufReader::new(
                    file_handle,
                ))),
                false => {
                    let mut buff = Vec::<u8>::new();

                    file_handle.read_to_end(&mut buff)?;

                    Ok(FileReadingReturnTypes::ReturnEntireFile(
                        String::from_utf8_lossy(&mut buff).to_string(),
                    ))
                }
            },
            Err(err) => {
                if panic_if_absent == true {
                    panic!("Error while opening the file")
                } else {
                    Err(err)
                }
            }
        }

        // if file_handle.unwrap().is_err() && panic_if_absent == true {
        //     println!("Some error while trying to read the file");
        //     Err(file_handle.err());
        // } else {
        //     match streamify {
        //         true => Ok(BufReader::new(file_handle.unwrap())),
        //         false => Ok(String::from_utf8_lossy(file_handle.unwrap())),
        //     }
        // }
    }

    // pub fn write_to_file(path_to_file: &str, panic_if_absent: bool) {}

    // TODO: UNCOMMENT THIS AFTER WRITING THE TEST CASES
    pub fn parse_to_json<json_blue_print>(
        path_to_file: &str,
    ) -> Result<json_blue_print, JSONErrorTypes>
    where
        json_blue_print: for<'a> Deserialize<'a>,
    {
        let file_handle = Utils::read_file(path_to_file, true, false).unwrap();

        let file_contents = match file_handle {
            FileReadingReturnTypes::ReturnBufferHandle(buffer) => {
                let mut string_collector: String = String::new();
                for line in buffer.lines() {
                    string_collector.push_str(&line.unwrap());
                }

                string_collector
            }
            FileReadingReturnTypes::ReturnEntireFile(file_as_string) => {
                // serde_json::from_str(&file_as_string)
                file_as_string
            }
        };

        match serde_json::from_str::<json_blue_print>(&file_contents) {
            Ok(deserialized_file) => Ok(deserialized_file),
            Err(err) => Err(JSONErrorTypes::JSONError(err)),
        }
    }
}

// test cases
// pass an invalid file location
// // pass a panic flag
// // dont pass a panic flag
// // pass a streamify flag
// // dont pass a streamify flag

// check for the case in which a directory is passed instead of a file

// #[test]
#[cfg(test)]
mod tests {

    use std::panic;

    use serde::Serialize;

    use super::*;

    static INVALID_FILE_LOCATION: &str = "";
    static VALID_FILE_LOCATION: &str =
        "/Users/nair-sreerag/codes/rn/tests/supporting_files/test_file.txt";
    static FOLDER_LOCATION: &str = "";
    static PERMISSION_DENIED_FILE_LOCATION: &str = "";
    static PERMISSION_DENIED_FOLDER_LOCATION: &str = "";

    static VALID_JSON_FILE_LOCATION: &str =
        "/Users/nair-sreerag/codes/rn/tests/supporting_files/test_json.json";

    #[test]
    #[should_panic(expected = "Error while opening the file")]
    fn read_from_an_invalid_location_with_panic_flag_set() {
        let read_file_result = Utils::read_file(INVALID_FILE_LOCATION, true, false);

        match read_file_result {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    #[test]
    #[should_panic(expected = "No such file or directory (os error 2)")]
    fn read_from_an_invalid_location_with_panic_flag_unset() {
        let read_file_result = Utils::read_file(INVALID_FILE_LOCATION, false, false);

        match read_file_result {
            Ok(_) => {
                // this case wont run
            }
            Err(err) => {
                std::panic!("{}", err.to_string());
            }
        }
    }

    // //  valid string locations after this point
    #[test]
    fn read_from_a_valid_location_with_streamify_flag_set() {
        let read_file_result = Utils::read_file(VALID_FILE_LOCATION, true, false).unwrap();

        match read_file_result {
            FileReadingReturnTypes::ReturnBufferHandle(buffer) => {
                self::panic!("Unexpected output");
            }
            FileReadingReturnTypes::ReturnEntireFile(entire_file) => {
                assert_eq!(entire_file.as_str(), "If you are seeing this message, then you are reading this properly.\nSome more text...\nLorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus lobortis sapien velit, sit amet bibendum enim maximus eget. Fusce ultricies malesuada libero, sit amet tristique quam fringilla eget. Ut ullamcorper mauris lectus, vel gravida ligula vehicula non. Etiam rhoncus finibus lectus, at cursus odio bibendum non. Pellentesque ex enim, commodo vel erat sed, ultricies aliquam sem. Fusce in purus justo. Nunc odio purus, ullamcorper sed dui ut, fringilla commodo arcu. In consectetur imperdiet lacus, non pharetra nulla laoreet facilisis. Morbi auctor lacus erat, eu volutpat enim iaculis eget. Sed vitae neque in ex tempus cursus. Suspendisse elementum, justo id porttitor efficitur, odio tellus gravida felis, a euismod nibh nulla sed quam. Ut in turpis sed arcu feugiat suscipit a vel elit. Nullam vel lacus non massa ullamcorper pharetra id eget turpis. Vestibulum nec sodales dui. Ut neque diam, convallis at efficitur ut, euismod vitae nulla. Aenean scelerisque at magna eget tempor.\n\nMaecenas quam nisl, interdum id arcu quis, consectetur elementum lacus. Integer sed vulputate purus, vel blandit ligula. Morbi felis neque, viverra eget ornare at, ullamcorper vitae diam. Mauris sed tortor efficitur velit euismod aliquet sed at enim. Morbi interdum mauris ut arcu elementum tristique. Praesent semper libero ac ante fringilla finibus. Phasellus ornare, lectus ut consectetur dignissim, ligula odio ultricies arcu, ac faucibus lacus felis vel neque. Nam nunc augue, convallis at augue finibus, placerat aliquam quam. Pellentesque suscipit neque sed convallis ullamcorper. Donec ultrices sollicitudin sagittis. Curabitur pretium pellentesque semper. Maecenas felis lectus, consequat eget pharetra quis, semper in orci. In at facilisis quam, non lacinia elit. Nunc accumsan congue ex, id finibus enim scelerisque eu. Ut in tempor purus, nec euismod massa.\n");
            }
        }
    }

    #[test]
    fn read_from_a_valid_location_with_streamify_flag_unset() {
        let read_file_result = Utils::read_file(VALID_FILE_LOCATION, true, false).unwrap();

        match read_file_result {
            FileReadingReturnTypes::ReturnBufferHandle(buffer) => {
                self::panic!("Unexpected output");
            }
            FileReadingReturnTypes::ReturnEntireFile(entire_file) => {
                assert_eq!(entire_file.as_str(), "If you are seeing this message, then you are reading this properly.\nSome more text...\nLorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus lobortis sapien velit, sit amet bibendum enim maximus eget. Fusce ultricies malesuada libero, sit amet tristique quam fringilla eget. Ut ullamcorper mauris lectus, vel gravida ligula vehicula non. Etiam rhoncus finibus lectus, at cursus odio bibendum non. Pellentesque ex enim, commodo vel erat sed, ultricies aliquam sem. Fusce in purus justo. Nunc odio purus, ullamcorper sed dui ut, fringilla commodo arcu. In consectetur imperdiet lacus, non pharetra nulla laoreet facilisis. Morbi auctor lacus erat, eu volutpat enim iaculis eget. Sed vitae neque in ex tempus cursus. Suspendisse elementum, justo id porttitor efficitur, odio tellus gravida felis, a euismod nibh nulla sed quam. Ut in turpis sed arcu feugiat suscipit a vel elit. Nullam vel lacus non massa ullamcorper pharetra id eget turpis. Vestibulum nec sodales dui. Ut neque diam, convallis at efficitur ut, euismod vitae nulla. Aenean scelerisque at magna eget tempor.\n\nMaecenas quam nisl, interdum id arcu quis, consectetur elementum lacus. Integer sed vulputate purus, vel blandit ligula. Morbi felis neque, viverra eget ornare at, ullamcorper vitae diam. Mauris sed tortor efficitur velit euismod aliquet sed at enim. Morbi interdum mauris ut arcu elementum tristique. Praesent semper libero ac ante fringilla finibus. Phasellus ornare, lectus ut consectetur dignissim, ligula odio ultricies arcu, ac faucibus lacus felis vel neque. Nam nunc augue, convallis at augue finibus, placerat aliquam quam. Pellentesque suscipit neque sed convallis ullamcorper. Donec ultrices sollicitudin sagittis. Curabitur pretium pellentesque semper. Maecenas felis lectus, consequat eget pharetra quis, semper in orci. In at facilisis quam, non lacinia elit. Nunc accumsan congue ex, id finibus enim scelerisque eu. Ut in tempor purus, nec euismod massa.\n");
            }
        }
    }

    #[test]
    fn read_the_entire_stream_and_compare() {
        let read_file_result = Utils::read_file(VALID_FILE_LOCATION, true, true).unwrap();

        match read_file_result {
            FileReadingReturnTypes::ReturnBufferHandle(buffer) => {
                let mut string_collector = String::new();

                for line in buffer.lines() {
                    let buffered_line = line.unwrap();

                    string_collector.push_str(buffered_line.as_str());
                }

                assert_eq!(string_collector, "If you are seeing this message, then you are reading this properly.Some more text...Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus lobortis sapien velit, sit amet bibendum enim maximus eget. Fusce ultricies malesuada libero, sit amet tristique quam fringilla eget. Ut ullamcorper mauris lectus, vel gravida ligula vehicula non. Etiam rhoncus finibus lectus, at cursus odio bibendum non. Pellentesque ex enim, commodo vel erat sed, ultricies aliquam sem. Fusce in purus justo. Nunc odio purus, ullamcorper sed dui ut, fringilla commodo arcu. In consectetur imperdiet lacus, non pharetra nulla laoreet facilisis. Morbi auctor lacus erat, eu volutpat enim iaculis eget. Sed vitae neque in ex tempus cursus. Suspendisse elementum, justo id porttitor efficitur, odio tellus gravida felis, a euismod nibh nulla sed quam. Ut in turpis sed arcu feugiat suscipit a vel elit. Nullam vel lacus non massa ullamcorper pharetra id eget turpis. Vestibulum nec sodales dui. Ut neque diam, convallis at efficitur ut, euismod vitae nulla. Aenean scelerisque at magna eget tempor.Maecenas quam nisl, interdum id arcu quis, consectetur elementum lacus. Integer sed vulputate purus, vel blandit ligula. Morbi felis neque, viverra eget ornare at, ullamcorper vitae diam. Mauris sed tortor efficitur velit euismod aliquet sed at enim. Morbi interdum mauris ut arcu elementum tristique. Praesent semper libero ac ante fringilla finibus. Phasellus ornare, lectus ut consectetur dignissim, ligula odio ultricies arcu, ac faucibus lacus felis vel neque. Nam nunc augue, convallis at augue finibus, placerat aliquam quam. Pellentesque suscipit neque sed convallis ullamcorper. Donec ultrices sollicitudin sagittis. Curabitur pretium pellentesque semper. Maecenas felis lectus, consequat eget pharetra quis, semper in orci. In at facilisis quam, non lacinia elit. Nunc accumsan congue ex, id finibus enim scelerisque eu. Ut in tempor purus, nec euismod massa.");
            }
            FileReadingReturnTypes::ReturnEntireFile(entire_file) => {
                assert_eq!(entire_file.as_str(), "If you are seeing this message, then you are reading this properly.\nSome more text...\nLorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus lobortis sapien velit, sit amet bibendum enim maximus eget. Fusce ultricies malesuada libero, sit amet tristique quam fringilla eget. Ut ullamcorper mauris lectus, vel gravida ligula vehicula non. Etiam rhoncus finibus lectus, at cursus odio bibendum non. Pellentesque ex enim, commodo vel erat sed, ultricies aliquam sem. Fusce in purus justo. Nunc odio purus, ullamcorper sed dui ut, fringilla commodo arcu. In consectetur imperdiet lacus, non pharetra nulla laoreet facilisis. Morbi auctor lacus erat, eu volutpat enim iaculis eget. Sed vitae neque in ex tempus cursus. Suspendisse elementum, justo id porttitor efficitur, odio tellus gravida felis, a euismod nibh nulla sed quam. Ut in turpis sed arcu feugiat suscipit a vel elit. Nullam vel lacus non massa ullamcorper pharetra id eget turpis. Vestibulum nec sodales dui. Ut neque diam, convallis at efficitur ut, euismod vitae nulla. Aenean scelerisque at magna eget tempor.\n\nMaecenas quam nisl, interdum id arcu quis, consectetur elementum lacus. Integer sed vulputate purus, vel blandit ligula. Morbi felis neque, viverra eget ornare at, ullamcorper vitae diam. Mauris sed tortor efficitur velit euismod aliquet sed at enim. Morbi interdum mauris ut arcu elementum tristique. Praesent semper libero ac ante fringilla finibus. Phasellus ornare, lectus ut consectetur dignissim, ligula odio ultricies arcu, ac faucibus lacus felis vel neque. Nam nunc augue, convallis at augue finibus, placerat aliquam quam. Pellentesque suscipit neque sed convallis ullamcorper. Donec ultrices sollicitudin sagittis. Curabitur pretium pellentesque semper. Maecenas felis lectus, consequat eget pharetra quis, semper in orci. In at facilisis quam, non lacinia elit. Nunc accumsan congue ex, id finibus enim scelerisque eu. Ut in tempor purus, nec euismod massa.\n");
            }
        }
    }

    #[test]
    #[should_panic(expected = "Error while opening the file")]
    fn read_from_folder() {
        let read_file_result: Result<FileReadingReturnTypes, Error> =
            Utils::read_file(FOLDER_LOCATION, true, false);

        match read_file_result {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    #[test]
    fn try_reading_a_permission_denied_file() {}

    #[test]
    fn try_reading_a_permission_denied_folder() {}

    // test cases for writing to a file

    // #[test]
    // fn write_to_a_valid_file() {}

    // #[test]
    // fn write_to_a_file_which_doesnt_exist() {}

    // #[test]
    // fn write_to_a_folder() {}

    // #[test]
    // fn write_to_a_readonly_file() {}

    // #[test]
    // fn write_to_an_invalid_file() {}

    // tc for parsing to a json

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct JSON_STRUCT_SCHEMA {
        a: u32,
        b: String,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct JSON_SCHEMA {
        x: u32,
        y: Vec<JSON_STRUCT_SCHEMA>,
        z: Vec<i32>,
    }

    // TEST CASES FOR fn parse_to_json()

    #[test]
    fn parse_to_a_valid_json<'a>() {
        let result = Utils::parse_to_json::<JSON_SCHEMA>(VALID_JSON_FILE_LOCATION);

        match result {
            Ok(file) => {
                assert_eq!(
                    file,
                    JSON_SCHEMA {
                        x: 2,
                        z: [1, 2, 3].to_vec(),
                        y: Vec::from([JSON_STRUCT_SCHEMA {
                            a: 123,
                            b: String::from("A name")
                        }])
                    }
                );
            }
            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }

    // #[test]
    // fn parse_to_an_invalid_json() {}
}
