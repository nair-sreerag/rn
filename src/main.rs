mod server;
pub mod utils;

use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    thread,
    time::Duration,
};

use rand::Rng;

use crate::server::{CoreServer, Server};

// pub mod config;
pub mod tester;

// use config::{actual, default};

fn main() {
    println!("Hello, World. Looks like we are starting off again!!");

    tester::zoro();

    let mut server = CoreServer::new("127.0.0.1", 10000, 40);

    server.create_threads();

    for b in server.start().incoming() {
        println!("Got an incoming request");

        let request = b.unwrap();

        server.execute(move || handle_connection(request))
    }
}

fn handle_connection(mut stream: TcpStream) {
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // let mut rng = rand::thread_rng();

    // let n1: u8 = rng.gen();

    // if (n1 % 2 == 0) {
    //     println!("Got {n1}; going to sleep for 5 seconds");
    //     thread::sleep(Duration::from_secs(5));
    // }

    // let response = "HTTP/1.1 200 OK\r\n\r\nHello World\r\n\r\n";

    // stream.write_all(response.as_bytes()).unwrap();

    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => {
            println!("Got a normal request");
            ("HTTP/1.1 200 OK", "hello.html")
        }
        "GET /sleep HTTP/1.1" => {
            println!("Got request for a thread to sleep");

            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // --snip--

    let contents = "Hello, World";
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
