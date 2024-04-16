use std::collections::HashMap;

use rand::Rng;

pub mod playground {}

#[derive(Debug)]
pub struct Printer {
    id: u32,
    job: Vec<u32>,
}

impl Printer {
    fn new(id: u32) -> Printer {
        Printer {
            id,
            job: Vec::new(),
        }
    }

    pub fn print_this(n: u32) {
        println!("{} is the current number being executed", n);
    }
}

#[derive(Debug)]
pub struct RoundRobin {
    threads: Vec<Printer>,
    record_keeper: HashMap<u32, u32>,
    next: u32,
}

impl RoundRobin {
    pub fn init(vec_number: u32) -> RoundRobin {
        let mut threads: Vec<Printer> = Vec::with_capacity(vec_number.try_into().unwrap());
        let mut hmap = HashMap::<u32, u32>::new();

        for i in 0..vec_number {
            threads.push(Printer::new(i));

            hmap.insert(i, 0);
        }

        RoundRobin {
            threads,
            next: 0,
            record_keeper: hmap,
        }
    }

    pub fn start(&mut self) {
        for i in 0..100 {
            let total_len = self.threads.len();

            self.threads[self.next as usize % total_len]
                .job
                .push(i.try_into().unwrap());

            let x = self.next % total_len as u32;

            *self.record_keeper.get_mut(&x).unwrap() += 1;

            let random = rand::thread_rng().gen_range(0..100);

            if random % 2 == 0 {
                let random_picker = rand::thread_rng().gen_range(0..self.threads.len());
                println!(
                    "got the random number {} so popping the value from {}",
                    random, random_picker
                );

                self.threads[random_picker].job.pop();

                self.update_thread_value(random_picker as u32);
            }

            if i % 25 == 0 {
                //  print the state of round_robin
                println!("printing the state for the {}th iteration", i);
                println!("{:?}", self);
            }
            self.next += 1;
        }

        println!("final values are {:?} ", self);
    }

    fn update_thread_value(&mut self, idx: u32) {
        println!("updating tallyman for {}", idx);
        println!("the state is {:?}", self.record_keeper);

        *self.record_keeper.get_mut(&idx).unwrap() -= 1;
    }
}

// create a struct
// initialize a variable with vector of size n
// keep a variable that tracks the next vector index to push the value to
// keep round robining and look at the

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    #[should_panic]
    pub fn it_works() {
        // let result = add(2, 2);
        panic!("zzz");
        assert_eq!(result, 5);
    }

    #[test]
    pub fn start_testing() {
        #[test]
        pub fn assign_tasks() {}

        #[test]
        pub fn validate_state() {}

        #[test]
        pub fn remove_tasks() {
            struct V {
                thread_id: u32,
                job_id: u32,
            }

            let checker = vec![
                V {
                    job_id: 10,
                    thread_id: 10,
                },
                V {
                    job_id: 10,
                    thread_id: 10,
                },
                V {
                    job_id: 10,
                    thread_id: 10,
                },
            ];
        }
    }
}
