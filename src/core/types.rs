use std::net::TcpStream;

pub type Job = Box<dyn FnOnce() + Send + 'static>;
