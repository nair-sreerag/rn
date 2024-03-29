// this module will house the various stages that can be used in the config files
// it will be stored in a hierarchial fashion
// eg > location block can proxy_pass, url rewriting, redirecting, caching, ratelimiting, error_page, auth_basic, add_header

use serde::{Deserialize, Serialize};

use self::rewrite::RewriteStage;

mod proxy_pass;
mod rewrite;

mod add_header;

#[derive(Debug, Clone, Serialize)]
pub enum Stage {
    Rewrite {
        // original_string: String,
        grouping_regex: String,
        replacement_regex: String,
        should_redirect: bool,
    },
    AddHeader, // add a header
    ProxyPass, // for reverse proxy
    AddAuthHeader,
    ModifyHeader,
    LimitConnections,
    ProxyCache,
    StaticFile,
    //
    //
    // FastCGI
}

impl<'de> Deserialize<'de> for Stage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

        let current_stage = value["action"].as_str().unwrap();

        match current_stage {
            "Rewrite" => Ok(Stage::Rewrite {
                // original_string: value["original_string"].as_str().unwrap().to_string(),
                grouping_regex: value["grouping_regex"].as_str().unwrap().to_string(),
                replacement_regex: value["replacement_regex"].as_str().unwrap().to_string(),
                should_redirect: value["should_redirect"].as_bool().unwrap(),
            }),

            "ProxyPass" => Ok(Stage::ProxyPass {}), // for reverse proxy

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

pub trait LocationStage {
    type StageInitArgs;

    fn init(args: Self::StageInitArgs) -> Self;

    fn process(self) -> String;
}
