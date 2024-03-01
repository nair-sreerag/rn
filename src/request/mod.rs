//  this module is all about collecting and
// manipulating the incoming request before sending it
// forward to the converned destination server

mod request;
use regex::Regex;
pub use request::CoreRequest;

pub struct ReplacementStruct<'a> {
    key: &'a str,
    value: &'a str,
}

pub trait Request {
    fn parse_request(stream: std::net::TcpStream) -> Vec<String>;

    // fn replace_headers(
    //     header: &str,
    //     expression: &str,
    //     replacement_values: Vec<ReplacementStruct>,
    // ) -> String;

    // this wont create a new entity; it will
    // just add the required headers and return it back
    fn add_headers(response_array: Vec<String>, header_string: Vec<String>) -> Vec<String>;
}
