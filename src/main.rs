mod block;
mod config;
mod core;
mod modules;
mod request;
mod routing_algos;

use ::config::{Config, File};
use config::*;
use modules::*;
use once_cell::sync::Lazy;
use routing_algos::ALGO_TYPES;

use crate::core::{CoreServer, Server};

pub static CONFIG: Lazy<CoreConfig> = Lazy::new(|| {
    const partial_path: &str = "conf/config.json";

    let mut abs_path = std::env::current_dir().unwrap();
    abs_path.push(partial_path);

    let full_path = match abs_path.to_str() {
        Some(p) => p,
        None => panic!("Something wrong with the path"),
    };

    let builder = Config::builder()
        .add_source(File::with_name(full_path))
        .build()
        .unwrap();

    builder.try_deserialize().unwrap()
});

fn main() {
    let x = CoreConfig::load();

    // let z: CoreConfig = x.unwrap();

    // println!("->>>> {:?}", z.items[1].first_name);

    // config::CoreConfig::CO

    // check if upstream array is provided, if yes parse them
    //  to parse - determine the server types - server tyoe only for now
    // default or health check

    println!("Hello, World");

    let server = core::CoreServer::new();

    server.start(CoreServer::handle_incoming_request)
}
