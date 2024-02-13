use std::{
    any,
    io::{BufRead, BufReader, Error, Write},
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
};

use crate::CoreThreadPool;

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

    pub fn handle_incoming_request_impl(mut stream: std::net::TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        // create a stream to the final destination - fd
        // forward the entire request to fd and wait for its response
        // when u get the response, return it back to the calling server

        // returning the response

        // let mut local_server_connection = TcpStream::connect("127.0.0.1:34254")?;

        println!("tolo this ->>> {:?}", http_request);

        // stream.write(http_request.);

        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = String::from("zz");
        let length = contents.len();

        stream
            .write_all(
                format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}").as_bytes(),
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

            let job = Box::new(move || executor_function(stream));

            self.sender.send(job).unwrap();
        }
    }

    fn get_permissible_limit() -> u32 {
        // TODO: change this

        20
    }

    fn handle_incoming_request_server(&self, mut stream: std::net::TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Request: {:#?}", http_request);
    }
}
