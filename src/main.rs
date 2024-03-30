mod block;
mod composer;
mod config;
mod core;
mod modules;
mod request;
mod routing_algos;

use ::config::{Config, File};
use config::CONFIG;
use modules::*;
use std::process;

use crate::core::{CoreServer, Server};

fn main() {
    // let z: CoreConfig = x.unwrap();

    println!("->>>> {:?}", CONFIG.configs[0].block_type);

    let pid = process::id();

    println!("process id is ->> {:?}", pid);
    // check if upstream array is provided, if yes parse them
    //  to parse - determine the server types - server tyoe only for now
    // default or health check

    println!("Hello, World");

    let server = core::CoreServer::new();

    server.start(CoreServer::handle_incoming_request)
}
