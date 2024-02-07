mod thread;

pub use thread::CoreThread;

pub trait Thread {
    // create a
    fn create(number_of_threads: u32);
}
