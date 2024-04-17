use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct ServerHttpsLayout {
    listen_on: i64,
    ssl_certificate_path: String,
    ssl_certificate_key_path: String,
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

        println!(
            "received these values from config for ServerHttpsLayout {}, {} and {}",
            listner_port, ssl_certificate_path, ssl_certificate_key_path
        );

        Ok(ServerHttpsLayout {
            listen_on: listner_port,
            ssl_certificate_path: String::from_str(ssl_certificate_path).unwrap(),
            ssl_certificate_key_path: String::from_str(ssl_certificate_key_path).unwrap(),
        })
    }
}
