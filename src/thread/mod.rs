mod thread;

pub use thread::CoreThread;

pub trait Thread {
    // create a thread pool
    fn create(number_of_threads: u32);
}
