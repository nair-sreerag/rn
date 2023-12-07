pub mod config;
pub mod logger;
pub mod server;
pub mod utils;

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use log::{error, info, trace, LevelFilter, Log};
use simple_logger::SimpleLogger;

fn main() {
    // println!("Hello, world!");
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_module_level("my_program", LevelFilter::Info)
        .init()
        .unwrap();

    info!("this is an info");
    trace!("this is an info");
    error!("this is an info");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for i in listener.incoming() {
        let stream = i.unwrap();

        println!("Connection established!!");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buff = BufReader::new(&mut stream);

    let http_request: Vec<_> = buff
        .lines()
        .map(|res| res.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let response = "HTTP/1.1 0 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
