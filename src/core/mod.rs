mod core;
use std::net::TcpListener;

pub use core::Core;

pub trait Server {
    // fn new(host: String, port: u32, s_port: u32, thread_count: usize) -> Self;

    fn start(&self) -> TcpListener;
}
