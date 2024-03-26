use crate::routing_algos::{RoutingAlgo, ROUTING_ALGORITHMS};

use self::upstream::ServerList;

mod upstream;

pub enum BLOCKS {
    UPSTREAM,
    LOCATION,
    SERVER,
    EVENTS,
}

pub trait CoreBlock<ROUTING_ALGORITHM: RoutingAlgo> {
    fn new(sl: Vec<ServerList>, algo: Option<ROUTING_ALGORITHMS>) -> Self;

    fn process(&self);
}
