use regex::Regex;

use crate::config::BlockType;

use super::CoreValidator;

pub struct CoreConfigValidator {}

impl CoreValidator for CoreConfigValidator {
    fn validate(config: &crate::config::CoreConfig) -> bool {
        for block_data in &config.configs {
            let block_type = &block_data.block_type;

            // check for each config provided
            match block_type {
                BlockType::Http => {
                    // the only thing i can find
                    // now is the proxy pass location module

                    // collect all cluster names
                    let all_cluster_names: Vec<String> = block_data
                        .clusters
                        .iter()
                        .map(|cluster_config| cluster_config.id.clone())
                        .collect();

                    for location_block in &block_data.locations {
                        for stage in &location_block.stages {
                            println!("stage name {:?}", stage);

                            match &stage.action {
                                crate::Stage::ProxyPass { url } => {
                                    // this will extract the domain name
                                    // and strip away the TLD and sub domain

                                    //TODO; THIS requires way more work

                                    let regex = r"(?<web_extension>http|https):\/\/(?<reverse_proxy_cluster_name>.*)";
                                    let regex_ready = Regex::new(regex).unwrap();

                                    let mut regex_extracted_cluster_name: String;

                                    match regex_ready.captures(&url[..].to_lowercase()) {
                                        Some(some_capture) => {
                                            regex_extracted_cluster_name = some_capture
                                                ["reverse_proxy_cluster_name"]
                                                .parse()
                                                .unwrap()
                                        }
                                        None => panic!(
                                            "Got an improperly constructed proxy_pass command"
                                        ),
                                    }

                                    if !all_cluster_names.contains(&regex_extracted_cluster_name) {
                                        panic!("cluster name provided in this stage doesn't exist");
                                    }
                                }

                                // TODO; handle other cases too
                                _ => todo!(),
                            }
                        }
                    }
                }

                BlockType::Events => todo!(),

                BlockType::Stream => todo!(),
            }
        }

        true
    }
}

#[cfg(test)]
pub mod tests {

    use crate::config::CoreConfig;

    use super::*;

    #[test]
    pub fn check_core_config_validator_for_http_block_type() {
        let config = CoreConfig::load();

        let core_validator = CoreConfigValidator::validate(&config.unwrap());

        assert_eq!(1, 1);
    }

    #[test]
    pub fn check_core_config_validator_for_events_block_type() {
        assert_eq!(1, 1);
    }

    #[test]
    pub fn check_core_config_validator_for_stream_block_type() {}
}
