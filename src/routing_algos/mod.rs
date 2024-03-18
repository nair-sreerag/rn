use crate::{channels::Channel, thread_pool::ThreadPool};

pub enum ALGO_TYPES {
    RoundRobin,
    LeastConnection,
    LeastRecentlyUsed,
    WeightedRoundRobin,
    Default,
}

pub mod default;

pub trait RoutingAlgo<
    JobType: Send + FnOnce() + 'static,
    ChannelType: Channel<JobType>,
    ThreadPoolType: ThreadPool<JobType>,
>
{
    fn new(channel_struct: ChannelType, thread_pool_struct: ThreadPoolType) -> Self;

    fn process_incoming_request(&self, executor_function: JobType) -> ();
}
