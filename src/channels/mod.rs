mod channels;

use std::sync::mpsc::{Receiver, Sender};

pub use channels::CoreChannel;
mod mock;

// pub use mock::MockChannel;

pub trait Channel<T> {
    fn create_mpsc(no_of_required_producers: u32) -> Self;
    fn get_producers(&self) -> &Vec<Sender<T>>;
    fn get_consumer(&self) -> &Receiver<T>;
}
