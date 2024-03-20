use std::sync::mpsc::Receiver;

use super::RoutingAlgo;

// pub struct RoundRobin {
//     senders: Vec<Sender<Job>>,
//     receivers: Vec<Receiver<Job>>,
// }

// impl RoutingAlgo for RoundRobin {
//     fn new(sender_count: u32, thread_count: u32) -> Self {
//         RoundRobin {}
//     }

//     fn process_job(&self, executor_function: crate::core::Job) {}
// }
