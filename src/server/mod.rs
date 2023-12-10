mod core;

pub use core::CoreServer;
use std::net::{TcpListener, TcpStream};

pub trait Server {
    fn init() -> TcpListener;
    fn handle_incoming_request(stream: TcpStream);
}
