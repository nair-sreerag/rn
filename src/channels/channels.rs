// use super::Channel;
use std::{sync::mpsc, thread};

use super::Channel;

pub struct CoreChannel {
    no_of_producers: u32,
}

// impl CoreChannel {}

impl Channel for CoreChannel {
    fn create_mpsc(&self) {
        &self.no_of_producers
    }
}
