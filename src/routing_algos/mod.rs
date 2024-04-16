use config::Value;
use serde::{Deserialize, Serialize};

use crate::core::Job;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum AlgoTypes {
    RoundRobin,
    LeastConnection,
    LeastRecentlyUsed,
    WeightedRoundRobin,
    Default,
}

pub mod default;
pub mod rr;
// pub mod lc;

pub trait RoutingAlgo {
    // initialize with all the required stuff
    // fn new(sender_count: u32, thread_count: u32) -> Self;

    fn process_job(&mut self, executor_function: Job);
}
