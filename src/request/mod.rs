//  this module is all about collecting and
// manipulating the incoming request before sending it
// forward to the converned destination server

mod request;
use std::{fmt::Display, io::Error};

use regex::Regex;
pub use request::CoreRequestParser;

pub struct ReplacementStruct<'a> {
    key: &'a str,
    value: &'a str,
}

enum PermissibleHeaderValues {
    Text(String),
    Number(u32),
}

pub struct Header {
    key: String,
    value: PermissibleHeaderValues,
}

impl Display for PermissibleHeaderValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(text) => write!(f, "{}", text),
            Self::Number(number) => write!(f, "{}", number.to_string()),
        }
    }
}

pub trait Request {
    // fn parse_request(stream: std::net::TcpStream) -> Vec<String>;

    // fn replace_headers(
    //     header: &str,
    //     expression: &str,
    //     replacement_values: Vec<ReplacementStruct>,
    // ) -> String;

    // this wont create a new entity; it will
    // just add the required headers and return it back
    // fn add_headers(&mut self, headers_to_add: Vec<Header>);

    // fn new(stream: &std::net::TcpStream) -> Self;

    fn replace_destination_uri(&mut self, new_uri: String) -> Result<bool, Error>;

    fn add_header(&mut self, new_header: String) -> Result<bool, Error>;

    // fn delete_header(&mut self) -> Result<bool, Error>;
}
