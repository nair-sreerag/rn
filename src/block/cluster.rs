use std::str::FromStr;

use crate::routing_algos::{default::DefaultRouting, RoutingAlgo};

use super::CoreBlock;

pub enum SERVER_TYPE {
    DEFAULT,
    HEALTH_CHECK,
    SERVER,
}

impl FromStr for SERVER_TYPE {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(SERVER_TYPE::DEFAULT),
            "health_check" => Ok(SERVER_TYPE::HEALTH_CHECK),
            "server" => Ok(SERVER_TYPE::SERVER),
            _ => panic!("Server type not supplied!"),
        }
    }
}

pub struct UpstreamServerStruct {
    server_type: SERVER_TYPE,
}

struct ServerClusterBlock {
    algo: Box<dyn RoutingAlgo>,
    servers: Vec<UpstreamServerStruct>,
}

impl ServerClusterBlock<T> {
    pub fn new(algorithm: String) -> Self {
        let algo = SERVER_TYPE::from_str(&algorithm).unwrap();

        ServerClusterBlock {
            algo: Box::new(DefaultRouting::new(1, 1)),
            servers: vec![UpstreamServerStruct {
                server_type: SERVER_TYPE::SERVER,
            }],
        }
    }
}

impl CoreBlock for ServerClusterBlock {
    fn process(&self) {}
}
