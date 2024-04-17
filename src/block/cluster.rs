use std::str::FromStr;

use crate::routing_algos::{default::DefaultRouting, RoutingAlgo};

use super::CoreBlock;

// this is used to determi
pub enum ServerType {
    Default,
    HealthCheck,
    Server,
}

impl FromStr for ServerType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(ServerType::Default),
            "health_check" => Ok(ServerType::HealthCheck),
            "server" => Ok(ServerType::Server),
            _ => panic!("Server type not supplied!"),
        }
    }
}

pub struct UpstreamServerStruct {
    server_type: ServerType,
}

struct ServerClusterBlock {
    algo: Box<dyn RoutingAlgo>,
    servers: Vec<UpstreamServerStruct>,
}

impl ServerClusterBlock {
    pub fn new(algorithm: String) -> Self {
        let algo = ServerType::from_str(&algorithm).unwrap();

        ServerClusterBlock {
            algo: Box::new(DefaultRouting::new(1, 1)),
            servers: vec![UpstreamServerStruct {
                server_type: ServerType::Server,
            }],
        }
    }
}

impl CoreBlock for ServerClusterBlock {
    fn process(&self) {}
}
