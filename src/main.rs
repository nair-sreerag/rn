mod block;
// mod collator;
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

use crate::{
    config::CoreConfig,
    core::{CoreServer, Server},
};

fn main() {
    println!("Hello, World... Starting the proxy server");

    let core_server = CoreServer::new();

    println!("config ->>> {:?}", CONFIG.configs);

    // HERE:
    // 1. validate the config format - DONE
    // 2. validate the config values
    // 3. initialize the specific routing algo
    // 4. send to the algo's process_job()

    // println!("{:?}", CONFIG.configs);

    core_server.start(CoreServer::handle_incoming_request)
}
