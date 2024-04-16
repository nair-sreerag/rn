mod block;
mod config;
mod core;
mod modules;
mod request;
mod routing_algos;
mod validator;

use ::config::{Config, File};
use config::CONFIG;
use modules::*;
use std::process;

use crate::core::{CoreServer, Server};

fn main() {
    println!("Hello, World... Starting the proxy server");

    // let z: CoreConfig = x.unwrap();

    let core_server = CoreServer::new();

    // HERE:
    // 1. validate the config format - DONE
    // 2. validate the config values
    // 3. initialize the specific routing algo
    // 4. send to the algo's process_job()

    // println!("{:?}", CONFIG.configs);

    println!("Hello, World");

    core_server.start(CoreServer::handle_incoming_request)
}
