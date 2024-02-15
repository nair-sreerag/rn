use std::io::{BufRead, BufReader};

use super::Request;

pub struct CoreRequest {}

impl CoreRequest {}

impl Request for CoreRequest {
    fn add_headers(request_array: Vec<String>, header_string: Vec<String>) -> Vec<String> {
        // TODO: implement this

        Vec::<String>::new()
    }

    fn transform_request(mut stream: std::net::TcpStream) -> Vec<String> {
        let buf_reader = BufReader::new(&mut stream);

        // this should handle different use cases
        //  for example when the POST request has a body,
        // u need to parse that separately after the perceived
        // terminator \r\n\r\n of a normal get request.
        //PS.  it marks the end of the headers section of any request

        // use the content length to find out how much the size of data is

        buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect()
    }
}
