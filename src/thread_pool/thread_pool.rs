use crate::core::JobWithoutBox;

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
    pub name: String,
    pub threads: Vec<thread::JoinHandle<JobType>>,
    // pub threads: Option<Vec<thread::JoinHandle<JobType>>>,
}

// remove this send here
impl<JobType: FnOnce() + Send + 'static> ThreadPool<JobType> for CoreThreadPool<JobType> {
    fn create_threads(
        // &mut self,
        no_of_threads: usize,
        // receiver: ReceiverTypes<Receiver<JobType>>,
        receiver: Receiver<JobType>,
    ) -> Self {
        let mut threads: Vec<thread::JoinHandle<JobType>> = Vec::new();

        // let resolved_receiver = match receiver {
        //     ReceiverTypes::MPSCType(job) => Arc::new(Mutex::new(job)),
        //     // ReceiverTypes::VecType(_) => panic!("This should not come in an actual application!"),
        //     _ => {
        //         panic!("This is an invalid state for the implementation");
        //     }
        // };

        if no_of_threads == 1 {
            // no need to do mutex... unnecessary overhead

            panic!("This is a case that needs to be handled properly")

            // TODO: implementation for this case
            // threads.push(thread::spawn(move || loop {
            //     let job = resolved_receiver.recv();
            // }))
        } else {
            let arced_mutex = Arc::new(Mutex::new(receiver));

            for i in 0..no_of_threads {
                let receiver = Arc::clone(&arced_mutex);

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
        }

        println!("initializing the threads");

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

    fn get_threads(&self) -> Vec<thread::JoinHandle<JobType>> {
        self.threads
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
