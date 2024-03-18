// use super::Channel;
use std::sync::mpsc::{self, Receiver, Sender};

use super::Channel;

// TODO: add a trait bound for JobType

pub struct CoreChannel<JobType> {
    no_of_required_producers: u32,
    consumer: Receiver<JobType>,
    producers: Vec<Sender<JobType>>,
    // pub no_of_required_producers: Option<u32>,
    // pub consumer: Option<Receiver<JobType>>,
    // pub producers: Option<Vec<Sender<JobType>>>,
}

// the mpsc producer
impl<JobType> Channel<JobType> for CoreChannel<JobType> {
    fn create_mpsc(no_of_required_producers: u32) -> Self {
        // &self.no_of_producers

        let mut no_of_required_producers = match no_of_required_producers {
            u32 => u32,
            _ => 1, // the default thing
        };

        let (producer, consumer) = mpsc::channel::<JobType>();

        let mut senders: Vec<Sender<JobType>> = Vec::new();

        for _ in 0..no_of_required_producers {
            senders.push(producer.clone())
        }

        // self
        CoreChannel {
            consumer,
            no_of_required_producers,
            producers: senders,
        }
    }
    fn get_consumer(&self) -> &Receiver<JobType> {
        &self.consumer
    }

    fn get_producers(&self) -> &Vec<Sender<JobType>> {
        &self.producers
    }
}

#[cfg(test)]
mod tests {}
