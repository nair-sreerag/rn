mod core;
mod types;

pub use core::*;
pub use types::*;

pub trait Server {
    // initializes with all the required things
    fn new() -> Self;

    // starts the server
    fn start<F: FnOnce(std::net::TcpStream) + Send + Copy + 'static>(&self, executor_function: F);

    //  used to check if the total permissible number of the threads
    // exceeds the total system capacity
    fn get_permissible_limit(&self) -> u32;
}
