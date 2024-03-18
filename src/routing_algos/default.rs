use std::{str::FromStr, sync::mpsc::Sender, thread};

use crate::{channels::Channel, thread_pool::ThreadPool};

use super::RoutingAlgo;

pub struct DefaultRouting<JobType> {
    producer: Sender<JobType>,
    name: String,
    threads: Vec<thread::JoinHandle<JobType>>,
}

impl<
        JobType: Send + FnOnce() + 'static,
        ChannelType: Channel<JobType>,
        ThreadPoolType: ThreadPool<JobType>,
    > RoutingAlgo<JobType, ChannelType, ThreadPoolType> for DefaultRouting<JobType>
{
    fn new(channel_struct: ChannelType, thread_pool_struct: ThreadPoolType) -> Self {
        let producer = channel_struct.get_producers();

        let threads = thread_pool_struct.get_threads();

        DefaultRouting {
            name: String::from("sdasd"),
            producer: producer[0],
            threads,
        }
    }

    fn process_incoming_request(&self, executor_function: JobType) -> () {
        // run this algo

        println!(":got a new job ----- ");

        self.producer.send(executor_function);
    }
}
