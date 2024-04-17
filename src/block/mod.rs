// WHAT DOES THIS MODULE DO??
// parses the level_0 keys and their values

mod cluster;
mod location;

pub use location::ServerLocationBlock;

pub enum BLOCKS {
    // Upstream, - isnt this the same thing as Cluster?
    Location,
    Cluster,
    Server,
    Event,
    Listener,
}

pub trait CoreBlock {
    // fn new<ROUTING_ALGORITHM: RoutingAlgo>(sl: Vec<i32>, algo: ROUTING_ALGORITHM) -> Self;

    fn process(&self);
}
