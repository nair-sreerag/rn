use std::{
    any,
    io::{BufRead, BufReader},
    net::TcpListener,
};

use super::Server;

pub struct CoreServer {
    host: String,
    port: u32,
    s_port: u32,
    thread_count: usize,
    // pool: Option<Vec<Wor>>,
    server_handle: Option<()>,
}

impl CoreServer {
    pub fn new(host: String, port: u32, s_port: u32, thread_count: usize) -> Self {
        println!("Intializing the server.");

        println!("Thread count is {}", thread_count);

        Self {
            host,
            port,
            s_port,
            thread_count,
            server_handle: Some(()),
        }
    }
}

impl Server for CoreServer {
    fn get_server_handle(&self) -> std::net::TcpListener {
        let complete_address: String = format!("{}:{}", self.host, self.port);

        let server_binding = TcpListener::bind(complete_address);

        server_binding.unwrap()
    }

    fn start(&self, server_handle: std::net::TcpListener) {
        println!(
            "Server is now listening on http://{}:{}",
            self.host, self.port
        );

        for l in server_handle.incoming() {
            let stream = l.unwrap();

            println!("Found a function");

            self.handle_incoming_request(stream);
        }
    }

    fn handle_incoming_request(&self, mut stream: std::net::TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Request: {:#?}", http_request);
    }

    fn get_permissible_limit() -> u32 {
        // TODO: change this

        20
    }
}
