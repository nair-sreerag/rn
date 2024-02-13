use super::Thread;
use std::thread;

pub struct CoreThread {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Thread for CoreThread {
    fn create(number_of_threads: u32) {}
}
