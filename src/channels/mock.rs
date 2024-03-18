use std::sync::mpsc::{self, Receiver, Sender};

use super::Channel;

type MockJobType = i32;

pub struct MockChannel<T> {
    pub no_of_required_producers: u32,
    pub consumer: Receiver<T>,
    pub producers: Vec<Sender<T>>,
}

impl<C> Channel<C> for MockChannel<C> {
    fn create_mpsc(no_of_required_producers: u32) -> Self {
        let mut producer_collector: Vec<C> = Vec::new();

        let (producer, consumer) = mpsc::channel::<C>();

        // if no_of_required_producers == 1 {
        // } else {
        //     // producer_collector.push("" as C);
        // }
        //
        let mut senders: Vec<Sender<C>> = Vec::new();

        for i in 0..no_of_required_producers {
            senders.push(producer.clone());
        }

        MockChannel {
            no_of_required_producers,
            consumer,
            producers: senders,
        }
    }

    fn get_consumer(&self) -> &std::sync::mpsc::Receiver<C> {
        &self.consumer
    }

    fn get_producers(&self) -> &Vec<std::sync::mpsc::Sender<C>> {
        &self.producers
    }
}

// EXAMPLE IMPLEMENTATION - WORKING

// use std::sync::mpsc;
// use std::thread;

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     // Producer
//     let producer = thread::spawn(move || {
//         let data = vec![1, 2, 3, 4, 5];
//         for x in data {
//             tx.send(x).unwrap();
//             println!("Sent {}", x);
//                         // thread::sleep(std::time::Duration::from_secs(2));

//         }
//     });
//     // Consumer
//     let consumer = thread::spawn(move || {
//         for received in rx {
//                         thread::sleep(std::time::Duration::from_secs(1));

//             println!("Received {}", received);
//         }
//     });
//     producer.join().unwrap();
//     consumer.join().unwrap();
// }

#[cfg(test)]
mod tests {

    use std::{thread, time::Duration};

    use rand::Rng;

    use super::*;

    static VEC_CAP: usize = 10;

    #[test]
    fn test_this_once() {
        use rand;

        let mock = MockChannel::<i32>::create_mpsc(5);

        let mut rng = rand::thread_rng();
        let sender = mock.get_producers();
        let receiver = mock.get_consumer();

        let generate_random_data_to_push: Vec<i32> =
            (0..VEC_CAP).map(|_| rng.gen_range(0..100)).collect();

        for i in 0..VEC_CAP {
            sender[0].send(generate_random_data_to_push[i]).unwrap();
            // thread::sleep(Duration::from_millis(1000));
        }

        let mut all_received_values = Vec::<i32>::new();

        let mut rx_count = 0;

        loop {
            for r in receiver.recv() {
                // let v = r;
                all_received_values.push(r);
                rx_count += 1;
            }

            if rx_count == VEC_CAP {
                break;
            }
        }

        assert_eq!(generate_random_data_to_push, all_received_values)
    }
}
