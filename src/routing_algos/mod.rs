pub mod rr;

pub enum ALGO_TYPES {
    RoundRobin,
    LeaseConnection,
    LeastRecentlyUsed,
}

pub trait RoutingAlgo {
    fn new() -> Self;

    fn start(&self) -> ();
}
