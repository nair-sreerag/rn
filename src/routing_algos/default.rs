use crate::channels::Channel;

use super::{RoutingAlgo, ThreadCollector};

pub struct DefaultRouting<ChannelType, ThreadPoolType> {
    name: String,
    consumer: ChannelType,
    threads: ThreadPoolType,
}

// this will take a struct which has 1 sender and 1 receiver
// and apply arc<mutex>
// to share the 1 receiver amongst multiple threads

impl<C, T> RoutingAlgo for DefaultRouting<C, T> {
    fn new<ChannelGeneric: Channel, TC>(
        mpsc: ChannelGeneric,
        thread_pool: ThreadCollector<TC>,
    ) -> Self {
        // let (sender, receiver) = mpsc;

        let t = match thread_pool {
            ThreadCollector::SimulatedThreadPool(p) => panic!("Simulation should not run here"),
            ThreadCollector::ActualThreadPool(a) => a,
        };

        // return a struct with all the required initializations
        // -
        // every thread should share the mutex receiver

        DefaultRouting {
            name: String::from("DEFAULT"),
            threads: t,
            consumer: mpsc.m,
        }
    }

    // fn start() {}

    // fn assign_to_thread() {}
}

#[cfg(test)]
mod tests {

    use super::*;
}
