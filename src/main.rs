// mod channels;
mod core;
mod pool;
mod request;
mod utils;

mod routing_algos;

pub use core::CoreServer;
use std::io::{BufRead, BufReader};

use crate::core::Server;
pub use pool::CoreThreadPool;

extern crate playground;

fn main() {
    println!("Hello, world!");

    let mut rr = playground::round_robin::init(5);

    rr.start();

    return;

    let core_server = CoreServer::new(String::from("localhost"), 10000, 443, 10);

    core_server.start(
        match core_server.get_server_handle() {
            Ok(server) => server,
            Err(error) => panic!("{}", error),
        },
        CoreServer::handle_incoming_request,
    );
}
