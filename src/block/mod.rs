use crate::routing_algos::RoutingAlgo;

pub enum Blocks {
    UPSTREAM,
    LOCATION,
    SERVER,
    EVENTS,
}

pub trait CoreBlock {
    // fn new<ROUTING_ALGORITHM: RoutingAlgo>(sl: Vec<i32>, algo: ROUTING_ALGORITHM) -> Self;

    fn process(&self);
}
