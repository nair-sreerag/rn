use super::{RoutingAlgo, ThreadCollector};

struct DefaultRouting {}

// this will take a struct which has 1 sender and 1 receiver
// and apply arc<mutex>
// to share the 1 receiver amongst multiple threads

impl RoutingAlgo for DefaultRouting {
    fn new<X>(mpsc: X, thread_pool: ThreadCollector) -> Self {
        DefaultRouting {}
    }

    // fn start() {}

    // fn assign_to_thread() {}
}

#[cfg(test)]
mod tests {

    use super::*;
}
