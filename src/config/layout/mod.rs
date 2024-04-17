use config::Value;
use serde::{Deserialize, Serialize};

use self::cluster::{ClusterConfigurationComposition, ClusterLayout};

use self::location::LocationConfigurationComposition;
use self::server_http::ServerHttpLayout;
use self::server_https::ServerHttpsLayout;

pub mod cluster;
pub mod location;
pub mod server_http;
pub mod server_https;

#[derive(Debug, Clone, Serialize)]
pub struct ConfigRootLevelComposition {
    pub block_type: BlockType,
    pub server_http: ServerHttpLayout,
    pub server_https: ServerHttpsLayout,
    pub clusters: Vec<ClusterConfigurationComposition>,
    pub locations: Vec<LocationConfigurationComposition>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BlockType {
    Http,
    // {
    //     // block_type: String::from(""),
    //     clusters: Vec<ClusterConfigurationComposition>,
    //     locations: Vec<LocationConfigurationComposition>,
    // },

    // TODO;
    Events,
    Stream,
}

impl<'de> Deserialize<'de> for ConfigRootLevelComposition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        let enum_value = value["block_type"].as_str().unwrap();
        match enum_value {
            "http" => Ok(ConfigRootLevelComposition {
                block_type: BlockType::Http,
                clusters: serde_json::from_value(value["clusters"].clone()).unwrap(),
                locations: serde_json::from_value(value["locations"].clone()).unwrap(),
                server_http: serde_json::from_value(value["server_http"].clone()).unwrap(),
                server_https: serde_json::from_value(value["server_https"].clone()).unwrap(),
            }),

            "events" => todo!(),
            "stream" => todo!(),

            _ => Err(serde::de::Error::custom(
                "Invalid block_type provided. Refer the docs for possible values.",
            )),
        }
    }
}
