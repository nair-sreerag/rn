use std::net::TcpListener;

use super::Server;

pub struct Core {
    host: String,
    port: u32,
    s_port: u32,
    thread_count: usize,
}

impl Core {
    pub fn new(host: String, port: u32, s_port: u32, thread_count: usize) -> Self {
        println!("Intializing the server.");

        println!("Thread count is {}", thread_count);

        Self {
            host,
            port,
            s_port,
            thread_count,
        }
    }
}

impl Server for Core {
    fn start(&self) -> std::net::TcpListener {
        println!("Server is now listening on port {}", self.port);

        let complete_address: String = format!("127.0.0.1:{}", self.port);

        let server_binding = TcpListener::bind(complete_address);

        server_binding.unwrap()
    }
}
