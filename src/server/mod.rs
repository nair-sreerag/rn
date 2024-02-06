mod core;

pub use core::CoreServer;
use std::net::{TcpListener, TcpStream};

pub trait Server {
    // this function will take in the number of
    // threads required to be spawned
    // among other parameters
    fn new(host: &str, port: u32, thread_count: usize) -> CoreServer;

    // this function will create said threads and return the server object

    fn create_threads(&mut self);

    //  this will start the server and make it listen on diff ports
    fn start(&self) -> TcpListener;

    // fn deploy(&mut self) -> CoreServer;
    fn execute<T>(&self, t: T)
    where
        T: FnOnce() + Send + 'static;
}
