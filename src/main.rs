// mod channels;
mod core;
mod pool;

pub use core::CoreServer;
use std::io::{BufRead, BufReader};

use crate::core::Server;
pub use pool::CoreThreadPool;

fn main() {
    println!("Hello, world!");

    let core_server = CoreServer::new(String::from("localhost"), 10000, 443, 10);

    core_server.start(
        match core_server.get_server_handle() {
            Ok(server) => server,
            Err(error) => panic!("{}", error),
        },
        handle_incoming_request,
    );
}

fn handle_incoming_request(mut stream: std::net::TcpStream) {
    println!("yoloman");
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}
