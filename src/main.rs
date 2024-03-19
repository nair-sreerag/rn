mod channels;
mod core;
mod request;
mod thread_pool;
mod utils;

pub use thread_pool::CoreThreadPool;

mod routing_algos;

pub use core::CoreServer;
// use std::io::{BufRead, BufReader};

// use crate::{core::Server, routing_algos::RoutingAlgo};

// extern crate playground;

use routing_algos::{
    default,
    // lc, lru, rr, wrr,
    ALGO_TYPES,
};

use crate::core::{Job, Server};

use crate::routing_algos::default::DefaultRouting;
use crate::routing_algos::RoutingAlgo;
use crate::thread_pool::ThreadPool;
use crate::{
    channels::{Channel, CoreChannel},
    //     core::Job,
    //     routing_algos::{RoutingAlgo, ThreadCollector},
    //     thread_pool::{CoreThreadPool, ReceiverTypes, ThreadPool},
};

fn main() {
    println!("Hello, world!");

    let algo: ALGO_TYPES = ALGO_TYPES::Default;

    // // check the number of threads that can be made
    // // and pass it to the routing algos below

    // initialize the mpsc struct
    // initialize the thread_pool
    // pass both to the selected algorithm

    const channel_core: CoreChannel<Job> = CoreChannel::<Job>::create_mpsc(5);

    const t: CoreThreadPool<Job> =
        CoreThreadPool::<Job>::create_threads(2, channel_core.get_consumer());

    const routing_algo: DefaultRouting<Job> = default::DefaultRouting::new(channel_core, t);

    match algo {
        ALGO_TYPES::Default => {
            // mpsc
            // let mut c = CoreChannel::<Job> {
            //     consumer: None,
            //     no_of_required_producers: Some(5),
            //     producers: None,
            // };

            // c.create_mpsc(1);

            // channel_core.get_producers::<Job>()
            // thread pool
            //
        }
        ALGO_TYPES::LeastConnection => {}
        ALGO_TYPES::RoundRobin => {} //routing_algo = rr::RoundRobin::new(),
        ALGO_TYPES::WeightedRoundRobin => {}
        ALGO_TYPES::LeastRecentlyUsed => {}
    }

    let core_server = CoreServer::<CoreChannel<Job>, CoreThreadPool<Job>, DefaultRouting<Job>>::new(
        String::from("localhost"),
        10000,
        443,
        10,
        routing_algo,
    );

    // core_server.start(
    //     match core_server.get_server_handle() {
    //         Ok(server) => server,
    //         Err(error) => panic!("{}", error),
    //     },
    //     CoreServer::handle_incoming_request,
    //     core_server.handle_incoming_request(),
    // );
    //
    core_server.start(
        match core_server.get_server_handle() {
            Ok(server) => server,
            Err(error) => panic!("{}", error),
        },
        // CoreServer::handle_incoming_request,
    )
}
