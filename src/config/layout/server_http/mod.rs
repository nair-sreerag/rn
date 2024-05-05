use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct ServerHttpLayout {
    listen_on: i64,

    // if this flag is used, then whatever request reaches this
    // server, rn will send out a 302 status code response with the
    // https port and url back to the client
    redirect: bool,
    server_names: Vec<String>,
}

impl<'de> Deserialize<'de> for ServerHttpLayout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

        let listener_port = value["listen_on"].as_i64().unwrap();
        let should_redirect = value["redirect"].as_bool().unwrap();

        let server_names: Vec<String> = match value["server_names"].as_array() {
            Some(array_of_domain_names) => array_of_domain_names
                .iter()
                // TODO: parse the domain name being passed here too
                .map(|s| s.to_string())
                .collect(),
            None => Vec::new(),
        };

        println!(
            "received these values from config for ServerHttpLayout {}, {} and {:?}",
            listener_port, should_redirect, server_names
        );

        Ok(ServerHttpLayout {
            listen_on: listener_port,
            redirect: should_redirect,
            server_names,
        })
    }
}
