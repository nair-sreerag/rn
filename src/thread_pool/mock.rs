// Every module should have this file which will define the mock version
// of the main functionality of that module so that it can be used
// for testing

// use crate::core::Job;

// use super::{ReceiverTypes, ThreadPool};

// // create a closure to mock the thread function
// //

// pub type CLOSURE_JOB = Box<dyn FnOnce() + Send + 'static>;

// | some_fn| {
//     println!("running a function now");
//     some_fn()
// }

// impl<CLOSURE_JOB> ThreadPool for MockCoreThreadPool<CLOSURE_JOB> {
//     fn create_threads<CT>(
//         no_of_threads_to_spawn: usize,
//         // consumer: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<crate::core::Job>>>,
//         consumer: ReceiverTypes<CT>, //Vec<CT>,
//     ) -> Self {
//         let threads: Vec<CT> = Vec::new();

//         // match consumer {
//         //  ConsumerTypes::ArcType(())
//         // ConsumerTypes::VecType()
//         // }

//         // TODO: How to do about this???
//         for i in 0..no_of_threads_to_spawn {
//             // threads.push();
//         }

//         MockCoreThreadPool {
//             mock_threads: threads,
//             name: String::from("Mock_Core_Thread_Pool"),
//         }
//     }
// }

// use std::{
//     str::FromStr,
//     sync::{mpsc::Receiver, Arc, Mutex},
// };

// use super::ThreadPool;

// pub struct MockCoreThreadPool<T> {
//     name: String,
//     mock_threads: Vec<T>,
// }

// impl<T> ThreadPool<T> for MockCoreThreadPool<T> {
//     fn create_threads<JobType>(
//         // &mut self,
//         no_of_threads_to_spawn: usize,
//         // receiver: super::ReceiverTypes<std::sync::mpsc::Receiver<JobType>>,
//         receiver: std::sync::mpsc::Receiver<JobType>,
//     ) -> Self {
//         // no_of_threads_to_spawn = match no_of_threads_to_spawn {
//         //     size => size,
//         //     _ => 1,
//         // };

//         // let shareable_receiver = Arc::new(Mutex::new(receiver));

//         // let mut receiver_collector: Vec = Vec::with_capacity(no_of_threads_to_spawn);

//         // for i in 0..no_of_threads_to_spawn {
//         //     receiver_collector.push(shareable_receiver.clone());
//         // }

//         // MockCoreThreadPool {
//         //     name: String::from_str("z"),
//         //     mock_threads: receiver_collector,
//         // }
//     }
// }
