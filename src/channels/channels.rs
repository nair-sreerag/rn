// use super::Channel;
use std::sync::mpsc::{self, Receiver, Sender};

use super::Channel;

pub struct CoreChannel<JobType> {
    no_of_required_producers: Option<u32>,
    consumer: Receiver<JobType>,
    producers: Vec<Sender<JobType>>,
}

// the mpsc producer
impl<JobType> Channel for CoreChannel<JobType> {
    fn create_mpsc(&self, no_of_required_producers: u32) -> Self {
        // &self.no_of_producers

        let mut no_of_required_producers = match no_of_required_producers {
            u32 => u32,
            _ => 1, // the default thing
        };

        let mut senders: Vec<Sender<JobType>> = Vec::new();

        let (sender, receiver) = mpsc::channel::<JobType>();

        for _ in 0..no_of_required_producers {
            senders.push(sender.clone())
        }

        CoreChannel {
            consumer: receiver,
            no_of_required_producers: Some(no_of_required_producers),
            producers: senders,
        }
    }
}

#[cfg(test)]
mod tests {}
