use super::RoutingAlgo;

pub struct RoundRobin {}

impl RoutingAlgo for RoundRobin {
    fn new() -> Self {
        RoundRobin {}
    }

    fn start(&self) -> () {}
}

#[cfg(test)]

pub mod tests {}
