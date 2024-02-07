// mod channels;
mod core;
// mod thread;

pub use core::CoreServer;

use crate::core::Server;

fn main() {
    println!("Hello, world!");

    let core_server = CoreServer::new(String::from("localhost"), 10000, 443, 10);

    core_server.start(match core_server.get_server_handle() {
        Ok(server) => server,
        Err(error) => panic!("{}", error),
    });
}
