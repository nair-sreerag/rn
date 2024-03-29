use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

use crate::core::Job;

use super::RoutingAlgo;

pub struct RoundRobin {
    senders: Vec<Sender<Job>>,
    receivers: Vec<JoinHandle<Job>>,
    received_request_count: usize,
}

impl RoundRobin {
    pub fn new(sender_count: u32, thread_count: u32) -> Self {
        // it should be a 1-1 relationship
        // i.e. 1 sender to 1 receiver

        // thread_count is basically useless here... or is it??

        if sender_count == 0 {
            panic!("Thread count for round robin cannot be 0!");
        }

        let mut sender_collector: Vec<Sender<Job>> = Vec::new();
        let mut receiver_collector: Vec<JoinHandle<Job>> = Vec::new();

        for _ in 0..sender_count {
            let (sender, receiver) = mpsc::channel::<Job>();

            sender_collector.push(sender);

            let new_thread = thread::spawn(move || loop {
                let job = receiver.recv().unwrap();

                job();
            });

            receiver_collector.push(new_thread);
        }

        RoundRobin {
            received_request_count: 0,
            receivers: receiver_collector,
            senders: sender_collector,
        }
    }
}

impl RoutingAlgo for RoundRobin {
    fn process_job(&mut self, executor_function: crate::core::Job) {
        let total_sender_length = self.senders.len();

        println!(
            "received_request_count {} and it goes to thread {}",
            self.received_request_count,
            self.received_request_count % total_sender_length
        );

        self.senders[self.received_request_count % total_sender_length]
            .send(executor_function)
            .unwrap();

        self.received_request_count += 1;
    }
}

// #[cfg(test)]
// pub mod tests {

//     use super::*;

//     #[test]
//     pub fn check_if_round_robin_works() {

//         // let round_robin = RoundRobin::new(10, 1);
//         // round
//     }

//     #[test]
//     #[should_panic(expected = "Thread count for round robin cannot be 0!")]
//     pub fn check_round_robin_for_negative_thread_count_value() {}

//     #[test]
//     #[should_panic(expected = "Thread count for round robin cannot be 0!")]
//     pub fn check_round_robin_for_zero_thread_count_value() {}
// }
