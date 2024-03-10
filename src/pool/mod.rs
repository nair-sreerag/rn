mod mock;
mod pool;

use std::sync::{mpsc::Receiver, Arc, Mutex};

pub use pool::CoreThreadPool;

pub use mock::MockCoreThreadPool;

use crate::core::Job;

pub trait ThreadPool {
    fn create_threads(no_of_threads_to_spawn: usize, consumer: Arc<Mutex<Receiver<Job>>>) -> Self;
}
