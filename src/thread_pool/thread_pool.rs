use crate::core::Job;

use super::{ReceiverTypes, ThreadPool};
use std::{
    process::id,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

pub struct CoreThreadPool<JobType> {
    name: String,
    threads: Vec<thread::JoinHandle<JobType>>,
}

// remove this send here
impl<JT: Send> ThreadPool for CoreThreadPool<JT> {
    fn create_threads<JobType: FnOnce() + Send + 'static>(
        no_of_threads: usize,
        receiver: ReceiverTypes<Receiver<JobType>>,
    ) -> Self {
        let mut threads: Vec<thread::JoinHandle<JT>> = Vec::new();

        let arcd_receiver = match receiver {
            ReceiverTypes::MPSCType(job) => Arc::new(Mutex::new(job)),
            ReceiverTypes::VecType(_) => panic!("This should not come in an actual application!"),
            _ => {
                panic!("This is an invalid state for the implementation");
            }
        };

        println!("initializing the threads");

        for i in 0..no_of_threads {
            let receiver = Arc::clone(&arcd_receiver);

            let thread_handle = thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!(
                    "request is currently being handled by thread - {:?}",
                    thread::current().id()
                );

                job();
            });

            threads.push(thread_handle)
        }

        CoreThreadPool {
            // TODO: this should be the name of config being taken from
            // the conf file
            // multiple servers , some with load-balancing capabilities
            // can be defined in the config.
            // this will take that name so that its easily identifiable
            name: String::from("yolo thread pool"),
            threads,
        }
    }
}

// impl CoreThreadPool {
// TODO: producer and receiver type
// pub fn new(
//     no_of_threads: usize,
//     // producer: Sender<()>,
//     consumer: Arc<Mutex<Receiver<Job>>>,
// ) -> Self {
//     let mut threads: Vec<thread::JoinHandle<Job>> = Vec::new();

//     println!("initializing the threads");
//     for i in 0..no_of_threads {
//         let receiver = Arc::clone(&consumer);

//         threads.push(thread::spawn(move || loop {
//             let job = receiver.lock().unwrap().recv().unwrap();

//             println!(
//                 "request is currently being handled by thread - {:?}",
//                 thread::current().id()
//             );

//             job();
//         }))
//     }

//     CoreThreadPool {
//         // TODO: this should be the name of config being taken from
//         // the conf file
//         // multiple servers , some with load-balancing capabilities
//         // can be defined in the config.
//         // this will take that name so that its easily identifiable
//         name: String::from("yolo thread pool"),
//         threads: None,
//         // producer,
//     }
// }
// }
