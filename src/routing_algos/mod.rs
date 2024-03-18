use crate::{channels::Channel, thread_pool::ThreadPool};

pub mod default;
// pub mod lc;
// pub mod lru;
// pub mod rr;
// pub mod wrr;

pub enum ALGO_TYPES {
    RoundRobin,
    LeastConnection,
    LeastRecentlyUsed,
    WeightedRoundRobin,
    Default,
}

pub enum ThreadCollector<TP> {
    SimulatedThreadPool(Vec<String>),
    ActualThreadPool(TP),
}

pub trait RoutingAlgo {
    // this will initialize the algo
    fn new<X: Channel, TC>(mpsc_channel: X, thread_pool: ThreadCollector<TC>) -> Self;

    // this will run it
    // fn start(&self) -> ();

    // fn handle_incoming_request() -> ();
    // fn assign_to_thread(// mut stream
    // ) -> bool;
}
