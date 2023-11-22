use crate::server::traits::IServer;
use std::net::{TcpListener, TcpStream};

pub struct CoreServer {
    http_port: Option<i32>,
    https_port: Option<i32>,
    host: Option<String>,
}

impl IServer for CoreServer {
    fn init() -> CoreServer {
        CoreServer {
            http_port: (),
            https_port: (),
            host: (),
        }
    }

    fn start() {}
}
