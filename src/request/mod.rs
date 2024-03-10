//  this module is all about collecting and
// manipulating the incoming request before sending it
// forward to the converned destination server

mod request;
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

pub trait Request {
    // fn parse_request(stream: std::net::TcpStream) -> Vec<String>;

    // fn replace_headers(
    //     header: &str,
    //     expression: &str,
    //     replacement_values: Vec<ReplacementStruct>,
    // ) -> String;

    // this wont create a new entity; it will
    // just add the required headers and return it back
    fn add_headers(&self, headers_to_add: Vec<Header>) -> Self;

    // fn delete_headers();
}
