use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

use crate::core::Job;

use super::RoutingAlgo;

pub struct DefaultRouting {
    sender: Sender<Job>,
    receivers: Vec<thread::JoinHandle<Job>>,
    no_of_senders: u32,
}

impl RoutingAlgo for DefaultRouting {
    fn new(sender_count: u32, thread_count: u32) -> Self {
        // default routing has a sender and n receivers
        // the receivers are all assigned to m threads - n : m
        // the sender receives the message and emits it
        // a random receiver picks it up and processes it
        // no specialised algorithm is used in this

        let (sender, receiver) = mpsc::channel::<Job>();

        let mut sender_collector: Vec<Sender<Job>> = Vec::new();
        let mut receiver_collector: Vec<thread::JoinHandle<Job>> = Vec::new();
        let arced_receiver = Arc::new(Mutex::new(receiver));

        for i in 0..thread_count {
            let cloned_receiver = Arc::clone(&arced_receiver);

            let thread = thread::spawn(move || loop {
                let job = cloned_receiver.lock().unwrap().recv().unwrap();

                println!(
                    "request is currently being handled by thread - {:?}",
                    thread::current().id()
                );

                job();
            });
        }

        DefaultRouting {
            sender,
            no_of_senders: sender_count,
            receivers: receiver_collector,
        }
    }

    fn process_job(&self, executor_function: Job) {
        println!("sending code to a random thread");

        self.sender.send(executor_function).unwrap();
    }
}
