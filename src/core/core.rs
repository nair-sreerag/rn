use std::{
    any,
    fs::read_to_string,
    io::{BufRead, BufReader, Error, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
};

use regex::Regex;
// Content-Length: 101

use crate::{request::{CoreRequest, Request}, CoreThreadPool};

use super::{Job, Server};

pub struct CoreServer {
    host: String,
    port: u32,
    s_port: u32,
    thread_count: usize,
    // pool: Option<Vec<Wor>>,
    server_handle: Option<()>,
    sender: mpsc::Sender<Job>,
    // receiver: Arc<Mutex<Receiver<()>>>,
    thread_pool: CoreThreadPool,
}

impl CoreServer {
    pub fn new(host: String, port: u32, s_port: u32, thread_count: usize) -> Self {
        println!("Intializing the server.");

        println!("Thread count is {}", thread_count);

        let (sender, receiver) = mpsc::channel::<Job>();

        let rx = Arc::new(Mutex::new(receiver));

        let thread_pool = CoreThreadPool::new(thread_count, rx);



        Self {
            host,
            port,
            s_port,
            thread_count,
            server_handle: Some(()),
            sender,
            // receiver: rx,
            thread_pool,
        }
    }

    pub fn handle_incoming_request(mut stream: std::net::TcpStream) {
        
        let dude = CoreRequest::new(&stream);

        println!("{:?}", dude);

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
]
        ;

        // println!("tolo this ->>> {:?}", http_request);

        // u need to basically
        // modify the above buf_reader and

        let mut response1: Vec<u8> = Vec::<u8>::new();
        local_server_connection
            .write(format!("{}\r\n\r\n",get_request.join("\r\n")).as_bytes())
            .unwrap();


        let resp = CoreRequest::new(&local_server_connection, );
        stream
            .write_all(            
                format!("{}\r\n\r\n{}", resp.headers.join("\r\n"), resp.body).as_bytes()
            )
            .unwrap()
    }
}

impl Server for CoreServer {
    fn get_server_handle(&self) -> Result<std::net::TcpListener, std::io::Error> {
        let complete_address: String = format!("{}:{}", self.host, self.port);

        let server_binding = TcpListener::bind(complete_address);

        let server_binding_status = match server_binding {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        };

        server_binding_status
    }

    fn start<F>(&self, server_handle: std::net::TcpListener, executor_function: F)
    where
        F: FnOnce(std::net::TcpStream) + Send + Copy + 'static,
    {
        println!(
            "Server is now listening on http://{}:{}",
            self.host, self.port
        );

        for l in server_handle.incoming() {
            println!("got an incoming",);
            let stream = l.unwrap();

            // self.handle_incoming_request(stream);

            executor_function(stream);

            // let job = Box::new(move || executor_function(stream));

            // self.sender.send(job).unwrap();
        }
    }

    fn get_permissible_limit() -> u32 {
        // TODO: change this

        20
    }

    // fn handle_incoming_request_server(&self, mut stream: std::net::TcpStream) {
    //     let buf_reader = BufReader::new(&mut stream);
    //     let http_request: Vec<_> = buf_reader
    //         .lines()
    //         .map(|result| result.unwrap())
    //         .take_while(|line| !line.is_empty())
    //         .collect();

    //     println!("Request: {:#?}", http_request);
    // }
}
