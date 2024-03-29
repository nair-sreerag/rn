use config::Value;
use serde::{Deserialize, Serialize};

use self::cluster::{ClusterConfigurationComposition, ClusterLayout};

use self::location::LocationConfigurationComposition;

pub mod cluster;
pub mod location;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BLOCK_TYPE {
    HTTP,
    // {
    //     // block_type: String::from(""),
    //     clusters: Vec<ClusterConfigurationComposition>,
    //     locations: Vec<LocationConfigurationComposition>,
    // },

    // TODO;
    EVENTS,
    STREAM,
}

impl<'de> Deserialize<'de> for ConfigRootLevelComposition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

        println!("value in BLOCK_TYPE deserializer {:?}", value);

        let enum_value = value["block_type"].as_str().unwrap();

        match enum_value {
            "http" => Ok(ConfigRootLevelComposition {
                block_type: BLOCK_TYPE::HTTP,
                clusters: serde_json::from_value(value["clusters"].clone()).unwrap(),
                locations: serde_json::from_value(value["location"].clone()).unwrap(),
            }),
            // "EVENTS" | "STREAM" => {}
            _ => Err(serde::de::Error::custom(
                "Invalid block_type provided. Refer the docs for possible values.",
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ConfigRootLevelComposition {
    block_type: BLOCK_TYPE,
    clusters: Vec<ClusterConfigurationComposition>,
    locations: Vec<LocationConfigurationComposition>,
}
