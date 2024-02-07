// mod channels;
mod core;
// mod thread;

pub use core::CoreServer;
use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

use crate::core::Server;

fn main() {
    println!("Hello, world!");

    let core_server = CoreServer::new(String::from("localhost"), 10000, 443, 10);

    core_server.start(core_server.get_server_handle());
}
