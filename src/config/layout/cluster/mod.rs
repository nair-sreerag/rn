use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::routing_algos::AlgoTypes;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HealthCheckLayout {
    // default value is true and if its true,
    // should not ping the server for health-check
    pub is_disabled: bool,

    // if nothing is provided or omitted,
    // use "base_url" as the uri
    // or use the entire url;
    // this is the entire health-check url and not the partial
    // uri because api url and health url can be entirely different
    pub url: Option<String>,

    // time in ms to determine after how much time
    // to determine if the nth health-check pass has failed
    pub timeout: i32,

    // how many times to try the health check
    // before marking is_active as false
    pub tries: Option<i32>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ClusterLayout {
    pub is_backup: bool,
    pub is_active: bool,
    pub base_url: String, // base url with port
    pub health_check: Option<HealthCheckLayout>,
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
    pub id: String, // the name that will be used in the reverse proxy
    pub algorithm: AlgoTypes,
    pub server_configs: Vec<ClusterLayout>,
}

impl<'de> Deserialize<'de> for ClusterConfigurationComposition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

        let enum_value = value["algorithm"].as_str().unwrap_or("default");

        println!(
            "running deserializer of ClusterConfigurationComposition ->>> {:?}",
            enum_value
        );

        match enum_value {
            // FIXME;
            // "RoundRobin" => Ok(),
            // "LeastConnection" => Ok(),
            // "LeastRecentlyUsed" => Ok(),
            // "WeightedRoundRobin" => Ok(),
            "default" => Ok(ClusterConfigurationComposition {
                id: value["identifier"].as_str().unwrap_or("zzz").to_string(),
                algorithm: AlgoTypes::Default,
                server_configs: serde_json::from_value(value["server_configs"].clone()).unwrap(),
            }),

            _ => {
                println!("I am here");

                Ok(ClusterConfigurationComposition {
                    id: value["algorithm"].as_str().unwrap_or("zzz").to_string(),
                    algorithm: AlgoTypes::Default,
                    server_configs: serde_json::from_value(value["server_configs"].clone())
                        .unwrap(),
                })
            }
        }
    }
}
