use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::Stage;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StageLayout {
    stage_type: Stage,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LocationConfigurationComposition {
    url: String,              // the sub api to match with
    stages: Vec<StageLayout>, // order matters
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
