// Every module should have this file which will define the mock version
// of the main functionality of that module so that it can be used
// for testing

use crate::core::Job;

use super::ThreadPool;

pub struct MockCoreThreadPool {
    name: String,
    threads: Vec<Job>,
}

impl ThreadPool for MockCoreThreadPool {
    fn create_threads(
        no_of_threads_to_spawn: usize,
        consumer: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<crate::core::Job>>>,
    ) -> Self {
        let threads: Vec<Job> = Vec::new();

        for i in 0..no_of_threads_to_spawn {
            threads.push(1)
        }

        MockCoreThreadPool {
            threads,
            name: String::from("Mock_Core_Thread_Pool"),
        }
    }
}
