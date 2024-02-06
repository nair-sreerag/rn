mod core;

pub use core::Core;

use crate::core::Server;

fn main() {
    println!("Hello, world!");

    let c = Core::new(String::from("host"), 80, 443, 10);

    c.start();
}
