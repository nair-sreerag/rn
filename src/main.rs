mod config;
mod core;
mod request;
mod routing_algos;

use ::config::{Config, File};
use config::CoreConfig;
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
    // read the config here
    // config::CoreConfig::CO();
    // *config::CONFIG;
    //

    println!("{:?}", CONFIG.last_name);

    // config::CoreConfig::CO

    let routing_algos: ALGO_TYPES = ALGO_TYPES::Default;

    match routing_algos {
        ALGO_TYPES::Default => {}
        ALGO_TYPES::LeastConnection => {}
        ALGO_TYPES::RoundRobin => {} //routing_algo = rr::RoundRobin::new(),
        ALGO_TYPES::WeightedRoundRobin => {}
        ALGO_TYPES::LeastRecentlyUsed => {}
    }

    println!("Hello, World");

    let server = core::CoreServer::new();

    server.start(CoreServer::handle_incoming_request)
}
