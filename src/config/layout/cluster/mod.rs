use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::routing_algos::ALGO_TYPES;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HealthCheckLayout {
    // default value is true and if its true,
    // should not ping the server for health-check
    is_disabled: bool,

    // if nothing is provided or omitted,
    // use "base_url" as the uri
    // or use the entire url;
    // this is the entire health-check url and not the partial
    // uri because api url and health url can be entirely different
    url: Option<String>,

    // time in ms to determine after how much time
    // to determine if the nth health-check pass has failed
    timeout: i32,

    // how many times to try the health check
    // before marking is_active as false
    tries: Option<i32>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ClusterLayout {
    is_backup: bool,
    is_active: bool,
    base_url: String, // base url with port
    health_check: Option<HealthCheckLayout>,
}

impl<'de> Deserialize<'de> for ClusterLayout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

        println!("running deserializer of ClusterLayout");

        let is_backup = value["is_backup"].as_bool().unwrap();
        let is_active = value["is_active"].as_bool().unwrap();
        let base_url = value["base_url"].as_str().unwrap().to_string();

        let health_check =
            serde_json::from_value::<Option<HealthCheckLayout>>(value["health_check"].clone())
                .unwrap();
        // {
        //     Some(value) => Some(serde_json::from_value(value["health_check"].clone()).unwrap()),
        //     // None => serde_json::json!({})
        //     None => None,
        // };

        println!("the value of health_check ->> {:?}", health_check);

        Ok(ClusterLayout {
            is_backup,
            is_active,
            base_url,
            health_check,
        })
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ClusterConfigurationComposition {
    identifier: String, // the name that will be used in the reverse proxy
    algorithm: ALGO_TYPES,
    server_configs: Vec<ClusterLayout>,
}

impl<'de> Deserialize<'de> for ClusterConfigurationComposition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

        let enum_value = value["algorithm"].as_str().unwrap();

        println!("running deserializer of ClusterConfigurationComposition");

        match enum_value {
            // FIXME;
            // "RoundRobin" => Ok(),
            // "LeastConnection" => Ok(),
            // "LeastRecentlyUsed" => Ok(),
            // "WeightedRoundRobin" => Ok(),
            "Default" => Ok(ClusterConfigurationComposition {
                identifier: value["identifier"].as_str().unwrap().to_string(),
                algorithm: ALGO_TYPES::Default,
                server_configs: serde_json::from_value(value["server_configs"].clone()).unwrap(),
            }),

            _ => Ok(ClusterConfigurationComposition {
                identifier: value["identifier"].as_str().unwrap().to_string(),
                algorithm: ALGO_TYPES::Default,
                server_configs: serde_json::from_value(value["server_configs"].clone()).unwrap(),
            }),
        }
    }
}
