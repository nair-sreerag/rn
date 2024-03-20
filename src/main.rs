mod core;
mod request;
mod routing_algos;

use routing_algos::ALGO_TYPES;

fn main() {
    let routing_algos: ALGO_TYPES = ALGO_TYPES::Default;

    match routing_algos {
        ALGO_TYPES::Default => {}
        ALGO_TYPES::LeastConnection => {}
        ALGO_TYPES::RoundRobin => {} //routing_algo = rr::RoundRobin::new(),
        ALGO_TYPES::WeightedRoundRobin => {}
        ALGO_TYPES::LeastRecentlyUsed => {}
    }

    println!("Hello, World");
}
