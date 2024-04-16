// WHAT DOES THIS MODULE DO??
// parses the level_0 keys and their values

use crate::routing_algos::RoutingAlgo;

mod cluster;
mod location;

pub enum BLOCKS {
    Upstream,
    Location,
    Server,
    Event,
    Cluster,
    Listener,
}

pub trait CoreBlock {
    // fn new<ROUTING_ALGORITHM: RoutingAlgo>(sl: Vec<i32>, algo: ROUTING_ALGORITHM) -> Self;

    fn process(&self);
}
