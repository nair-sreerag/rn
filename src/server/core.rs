use std::{
    io::{prelude::*, BufRead, BufReader},
    net::TcpListener,
};

use super::Server;

pub struct CoreServer {}

impl Server for CoreServer {
    fn init() -> TcpListener {
        println!("Wow this is new!");

        let binding = TcpListener::bind("127.0.0.1:7000").unwrap();

        binding
    }

    fn handle_incoming_request(mut stream: std::net::TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();

        if request_line == "GET / HTTP/1.1" {
            // let http_request: Vec<_> = buf_reader
            //     .lines()
            //     .map(|result| result.unwrap())
            //     .take_while(|line| !line.is_empty())
            //     .collect();

            // println!("Request: {:#?}", http_request);

            let response = "HTTP/1.1 200 OK\r\n\r\n Hello World\r\n\r\n";

            stream.write_all(response.as_bytes()).unwrap();
        } else {
        }
    }
}

// pub mod core_server {

//     pub fn init() {
//         println!("Initializing the server");
//     }

//     pub fn start_server() {
//         println!("Starting the server");
//     }
// }
