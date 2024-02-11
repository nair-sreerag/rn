use std::{
    any,
    io::{BufRead, BufReader, Error},
    net::TcpListener,
    sync::mpsc,
    sync::{mpsc::Receiver, mpsc::Sender, Arc, Mutex},
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

    fn handle_incoming_request(&self, mut stream: std::net::TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Request: {:#?}", http_request);
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
}
