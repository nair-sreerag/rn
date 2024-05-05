use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct ServerHttpsLayout {
    listen_on: i64,
    ssl_certificate_path: String,
    ssl_certificate_key_path: String,
    server_names: Vec<String>,
}

impl<'de> Deserialize<'de> for ServerHttpsLayout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

        let listner_port = value["listen_on"].as_i64().unwrap();
        let ssl_certificate_path = value["ssl_certificate_path"].as_str().unwrap();
        let ssl_certificate_key_path = value["ssl_certificate_key_path"].as_str().unwrap();

        let server_names: Vec<String> = match value["server_names"].as_array() {
            Some(array_of_domain_names) => array_of_domain_names
                .iter()
                // TODO: parse the domain name being passed here too
                .map(|s| s.to_string())
                .collect(),
            None => Vec::new(),
        };

        println!(
            "received these values from config for ServerHttpsLayout {}, {}, {} and {:?}",
            listner_port, ssl_certificate_path, ssl_certificate_key_path, server_names
        );

        Ok(ServerHttpsLayout {
            listen_on: listner_port,
            ssl_certificate_path: String::from_str(ssl_certificate_path).unwrap(),
            ssl_certificate_key_path: String::from_str(ssl_certificate_key_path).unwrap(),
            server_names,
        })
    }
}
