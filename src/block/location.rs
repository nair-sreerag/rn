// this will parse the location block that is defined in the config file
// to make the required changes to the url

// it will have a series of stages that will transform the incoming request as
// per each stage

use std::str::FromStr;

use crate::{request::CoreRequestParser, LocationStage, Stage};

use super::CoreBlock;

pub struct ServerLocationBlock {
    url: String,
    stages: Option<Vec<Stage>>,
}

impl <R : LocationStage> CoreBlock for ServerLocationBlock {
    fn process(&self, parsed_incoming_request: <R>) {
        let url = ServerLocationBlock {
            url: String::from_str("this is a url").unwrap(),
            stages: None,
        };
    }
}
