mod server;
use crate::server::{CoreServer, Server};

fn main() {
    println!("Hello, World. Looks like we are starting off again!!");

    let bind = CoreServer::init();

    for b in bind.incoming() {
        println!("Connection established");

        let stream = b.unwrap();

        CoreServer::handle_incoming_request(stream);
    }
}
