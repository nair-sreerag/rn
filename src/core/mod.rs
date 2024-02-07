mod core;
use std::net::{TcpListener, TcpStream};

pub use core::CoreServer;

pub trait Server {
    // starts the server
    fn start(&self, server_handle: std::net::TcpListener);

    // returns the server handle because y not?
    fn get_server_handle(&self) -> TcpListener;

    //  used to check if the total permissible number of the threads
    // exceeds the total system capacity
    fn get_permissible_limit() -> u32;

    fn handle_incoming_request(&self, stream: TcpStream);
}
