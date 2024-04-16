use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use crate::{
    request::CoreRequestParser,
    routing_algos::{default::DefaultRouting, rr::RoundRobin, RoutingAlgo},
    // validator::CoreValidator,
};

use super::{Job, Server};

pub struct CoreServer {
    host: String,
    port: u32,

    thread_count: Option<u32>,
}

impl CoreServer {
    pub fn handle_incoming_request(mut stream: std::net::TcpStream) {
        let parsed_incoming_request = CoreRequestParser::new(&stream);

        println!("{:?}", parsed_incoming_request);

        let mut local_server_connection = TcpStream::connect("127.0.0.1:35577").unwrap();

        let get_request = vec![

                "GET /hello-world HTTP/1.1",
                "Host: 127.0.0.1:35577",
                "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
                "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
                "Accept-Language: en-US,en;q=0.5",
                "Accept-Encoding: gzip, deflate, br",
                "DNT: 1",
                "Connection: keep-alive",
                "Upgrade-Insecure-Requests: 1",
                "Sec-Fetch-Dest: document",
                "Sec-Fetch-Mode: navigate",
                "Sec-Fetch-Site: none",
                "Sec-Fetch-User: ?1",
                "Cache-Control: max-age=0",
            ];

        // println!("tolo this ->>> {:?}", http_request);

        // u need to basically
        // modify the above buf_reader and

        let mut response1: Vec<u8> = Vec::<u8>::new();
        local_server_connection
            .write(format!("{}\r\n\r\n", get_request.join("\r\n")).as_bytes())
            .unwrap();

        let resp = CoreRequestParser::new(&local_server_connection);
        stream
            .write_all(format!("{}\r\n\r\n{}", resp.headers.join("\r\n"), resp.body).as_bytes())
            .unwrap()
    }
}

impl Server for CoreServer {
    fn new() -> Self {
        CoreServer {
            // TODO;
            host: String::from("localhost"),
            port: 9000,
            thread_count: None,
        }
    }

    fn start<F>(&self, executor_function: F)
    where
        F: FnOnce(std::net::TcpStream) + Send + Copy + 'static,
    {
        let complete_address: String = format!("{}:{}", self.host, self.port);

        let server_binding = TcpListener::bind(complete_address).unwrap();

        println!(
            "Server is now listening on http://{}:{}",
            self.host, self.port
        );

        //TODO; remove this
        // let mut default_algo = DefaultRouting::new(1, 5);
        let mut default_algo = RoundRobin::new(100, 5);

        for incoming_connection in server_binding.incoming() {
            println!("Got an incoming request");

            let stream = incoming_connection.unwrap();

            let job = Box::new(move || executor_function(stream));

            // let core_validator = CoreValidator::validate(config);

            default_algo.process_job(job)
        }
    }

    fn get_permissible_limit(&self) -> u32 {
        // TODO; compute this dynamically

        30;

        todo!()
    }
}
