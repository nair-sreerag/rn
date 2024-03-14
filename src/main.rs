mod channels;
mod core;
mod thread_pool;
// mod request;
// mod utils;
// pub use thread_pool::CoreThreadPool;

mod routing_algos;

// pub use core::CoreServer;
// use std::io::{BufRead, BufReader};

// use crate::{core::Server, routing_algos::RoutingAlgo};

// extern crate playground;

// use routing_algos::{default, lc, lru, rr, wrr, ALGO_TYPES};

fn main() {
    // println!("Hello, world!");

    // let algo: ALGO_TYPES = ALGO_TYPES::Default;

    // let mut routing_algo;

    // // check the number of threads that can be made
    // // and pass it to the routing algos below

    // match algo {
    //     ALGO_TYPES::Default => {}
    //     ALGO_TYPES::LeastConnection => {}
    //     ALGO_TYPES::RoundRobin => routing_algo = rr::RoundRobin::new(),
    //     ALGO_TYPES::WeightedRoundRobin => {}
    //     ALGO_TYPES::LeastRecentlyUsed => {}
    // }

    // let core_server = CoreServer::new(String::from("localhost"), 10000, 443, 10, routing_algo);

    // core_server.start(
    //     match core_server.get_server_handle() {
    //         Ok(server) => server,
    //         Err(error) => panic!("{}", error),
    //     },
    //     CoreServer::handle_incoming_request,
    // core_server.handle_incoming_request(),
    // );
}
