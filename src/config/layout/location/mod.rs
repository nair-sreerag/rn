use std::str::FromStr;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::Stage;

#[derive(Debug, Serialize, Clone)]
pub struct StageLayout {
    pub action: Stage,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LocationConfigurationComposition {
    pub match_sub_uri: String,    // the sub api to match with
    pub stages: Vec<StageLayout>, // order matters
}

// impl Serialize for regex::Regex {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         serializer.serialize_str(&self.as_str())
//     }
// }

// impl Deserialize for Regex {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         deserializer.deserialize_str()
//     }
// }

impl<'de> Deserialize<'de> for StageLayout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

        // this will throw error in the upcoming match
        let current_stage = value["action"].as_str().unwrap_or("");

        match current_stage {
            "rewrite" => Ok(StageLayout {
                action: Stage::Rewrite {
                    // original_string: value["original_string"].as_str().unwrap().to_string(),
                    grouping_regex: value["grouping_regex"].as_str().unwrap().to_string(),
                    replacement_regex: value["replacement_regex"].as_str().unwrap().to_string(),
                    should_redirect: value["should_redirect"].as_bool().unwrap(),
                },
            }),

            "proxy_pass" => Ok(StageLayout {
                action: Stage::ProxyPass {
                    url: String::from_str("s").unwrap(),
                },
            }), // for reverse proxy

            // "AddHeader" => Ok(()), // add a header

            // "AddAuthHeader" => Ok(),

            // "ModifyHeader" => Ok(),

            // "LimitConnections" => Ok(),

            // "ProxyCache" => Ok(),

            // "StaticFile" => Ok(),
            _ => Err(serde::de::Error::custom("Invalid stage provided.")),
        }
    }
}
