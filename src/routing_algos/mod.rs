use crate::pool::ThreadPool;

pub mod default;
pub mod lc;
pub mod lru;
pub mod rr;
pub mod wrr;

pub enum ALGO_TYPES {
    RoundRobin,
    LeastConnection,
    LeastRecentlyUsed,
    WeightedRoundRobin,
    Default,
}

pub trait RoutingAlgo {
    fn new<ThreadPoolType: ThreadPool>(thread_pool: ThreadPoolType) -> Self;

    fn start(&self) -> ();

    // fn handle_incoming_request() -> ();
    fn assign_to_thread();
}
