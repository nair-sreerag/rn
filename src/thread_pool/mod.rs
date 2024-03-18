// mod mock;
// pub use mock::MockCoreThreadPool;
mod thread_pool;

use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    thread,
};

pub use thread_pool::CoreThreadPool;

use crate::core::JobWithoutBox;

pub enum ReceiverTypes<J> {
    MPSCType(J),
    // need a solid implementation for this
    // VecType(Vec<J>),
}

// this will create the receiver threads
// and add the receiver in them
// since threads need something to run something on
//

pub trait ThreadPool<JobType: Send + FnOnce() + 'static> {
    // fn create_threads(no_of_threads_to_spawn: usize, consumer: Arc<Mutex<Receiver<Job>>>) -> Self;
    // fn create_threads<JobType: Into<Send + FnOnce() + 'static> + Into<u32>>(
    fn create_threads(
        // &mut self,
        no_of_threads_to_spawn: usize,
        receiver: Receiver<JobType>,
    ) -> Self;

    // TODO: convert to reference if necessary
    fn get_threads(&self) -> Vec<thread::JoinHandle<JobType>>;
}
