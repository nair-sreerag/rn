use crate::core::Job;

use super::ThreadPool;
use std::{
    process::id,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

//  this contains all the threads and the
// mpsc code
pub struct CoreThreadPool {
    name: String,
    threads: Vec<thread::JoinHandle<Job>>,
    // producer: Sender<()>,
}

impl CoreThreadPool {
    // TODO: producer and receiver type
    pub fn new(
        no_of_threads: usize,
        // producer: Sender<()>,
        consumer: Arc<Mutex<Receiver<Job>>>,
    ) -> Self {
        let mut threads: Vec<thread::JoinHandle<Job>> = Vec::new();

        println!("initializing the threads");
        for i in 0..no_of_threads {
            let receiver = Arc::clone(&consumer);

            threads.push(thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!(
                    "request is currently being handled by thread - {:?}",
                    thread::current().id()
                );

                job();
            }))
        }

        CoreThreadPool {
            // TODO: this should be the name of config being taken from
            // the conf file
            // multiple servers , some with load-balancing capabilities
            // can be defined in the config.
            // this will take that name so that its easily identifiable
            name: String::from("yolo thread pool"),
            threads,
            // producer,
        }
    }
}
