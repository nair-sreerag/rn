mod pool;

pub use pool::CoreThreadPool;

pub trait ThreadPool {
    fn create_threads();

    fn create_mpsc();
}
