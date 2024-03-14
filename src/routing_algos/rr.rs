use std::sync::{mpsc::Receiver, Arc, Mutex};

use crate::{core::Job, pool::ThreadPool, CoreThreadPool};

use super::RoutingAlgo;

pub struct Thread {
    // TODO: THIS WHOLE THING GETS CHANGED
    thread_id: u32,
    thread: String,
    request_queue: Vec<String>,
}

pub struct RoundRobin {
    pool: Vec<Thread>,
    request_counter: u32,
    total_threads_in_use: u32,
}

impl RoutingAlgo for RoundRobin {
    fn new<X>(thread_pool: X) -> Self {
        let mut thread_initializer: Vec<Thread> = Vec::new();

        // for i in 0..thread_pool.len() {
        //     thread_initializer.push(Thread {
        //         thread: thread_pool[i].clone(),
        //         thread_id: i as u32,
        //         request_queue: Vec::<String>::new(),
        //     })
        // }

        RoundRobin {
            total_threads_in_use: thread_pool.len() as u32,
            request_counter: 0,
            pool: thread_initializer,
        }
    }

    fn start(&self) -> () {}

    fn assign_to_thread() {}
}

#[cfg(test)]

mod tests {

    // check the most normal case - no of threads - 4
    // the threads should wait for a specific point of time simulating the
    //

    use super::*;

    #[test]
    fn x() {}

    // system should have a flag that takes in the duration to wait for every thread - basically a simulation for
    // server wait time....
    // pass random values
    // send 100 jobs so that they are round robin'd
    // check the
}
