mod channels;

use std::sync::mpsc::{Receiver, Sender};

pub use channels::CoreChannel;
mod mock;

// pub use mock::MockChannel;

pub trait Channel<JobType> {
    fn create_mpsc(no_of_required_producers: u32) -> Self;

    // TODO: convert to reference if necessary
    fn get_producers(&self) -> Vec<Sender<JobType>>;
    fn get_consumer(&self) -> Receiver<JobType>;
}

// should take the no of producers and provide a struct
// with the n producers and 1 consumer... does it do that?? yes
// it also has getters for producer and consumer so that it can be used outside -- this might need refactoring in the future
