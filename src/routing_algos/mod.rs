use crate::core::Job;

pub enum ALGO_TYPES {
    RoundRobin,
    LeastConnection,
    LeastRecentlyUsed,
    WeightedRoundRobin,
    Default,
}

pub mod default;
pub mod rr;

pub trait RoutingAlgo {
    // initialize with all the required stuff
    fn new(sender_count: u32, thread_count: u32) -> Self;
    fn process_job(&self, executor_function: Job);
}
