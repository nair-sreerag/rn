use super::ThreadPool;
use std::{
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
    threads: Vec<thread::JoinHandle<()>>,
    // producer: Sender<()>,
}

impl CoreThreadPool {
    // TODO: producer and receiver type
    pub fn new(
        no_of_threads: usize,
        // producer: Sender<()>,
        consumer: Arc<Mutex<Receiver<()>>>,
    ) -> Self {
        let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();

        // TODO: see to its type

        for i in 0..no_of_threads {
            let receiver = Arc::clone(&consumer);

            threads.push(thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                // FIXME: this is like this because of the type being passed in JoinHandle?
                job;
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
