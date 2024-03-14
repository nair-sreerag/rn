mod mock;
mod thread_pool;

use std::sync::{mpsc::Receiver, Arc, Mutex};

pub use thread_pool::CoreThreadPool;

pub use mock::MockCoreThreadPool;

use crate::core::JobWithoutBox;

pub enum ReceiverTypes<J> {
    MPSCType(J),

    // need a solid implementation for this
    VecType(Vec<J>),
}

pub trait ThreadPool {
    // fn create_threads(no_of_threads_to_spawn: usize, consumer: Arc<Mutex<Receiver<Job>>>) -> Self;
    fn create_threads<JobType: Send + FnOnce() + 'static>(
        no_of_threads_to_spawn: usize,
        receiver: ReceiverTypes<Receiver<JobType>>,
    ) -> Self;
}
