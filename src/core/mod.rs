mod core;
use std::net::{TcpListener, TcpStream};

mod types;

pub use core::CoreServer;
pub use types::Job;

pub trait Server {
    // starts the server
    fn start<F: FnOnce(std::net::TcpStream) + Send + Copy + 'static>(
        &self,
        server_handle: std::net::TcpListener,
        request_handler: F,
    );

    // returns the server handle because y not?
    fn get_server_handle(&self) -> Result<std::net::TcpListener, std::io::Error>;

    //  used to check if the total permissible number of the threads
    // exceeds the total system capacity
    fn get_permissible_limit() -> u32;

    fn handle_incoming_request_server(&self, stream: TcpStream);
}
