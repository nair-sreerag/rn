use regex::Regex;

use crate::config::BLOCK_TYPE;

use super::CoreValidator;

pub struct CoreConfigValidator {}

impl CoreValidator for CoreConfigValidator {
    fn validate(config: &crate::config::CoreConfig) -> bool {
        for block_data in &config.configs {
            let block_type = &block_data.block_type;

            // check for each config provided
            match block_type {
                BLOCK_TYPE::HTTP => {
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
                            match &stage.action {
                                crate::Stage::ProxyPass { url } => {
                                    // this will extract the domain name
                                    // and strip away the TLD and sub domain

                                    //TODO; THIS requires way more work

                                    let regex = r"(?<web_extension>http|https)\:\/\/(?<reverse_proxy_name>.*)";
                                    let regex_ready = Regex::new(regex).unwrap();

                                    let mut regex_extracted_cluster_name: String;

                                    match regex_ready.captures(&url[..].to_lowercase()) {
                                        Some(some_capture) => {
                                            regex_extracted_cluster_name =
                                                some_capture[""].parse().unwrap()
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

                BLOCK_TYPE::EVENTS => {}

                BLOCK_TYPE::STREAM => {}
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
    pub fn check_core_configurator_for_http_block_type() {
        let config = CoreConfig::load();

        let core_validator = CoreConfigValidator::validate(&config.unwrap());

        assert_eq!(1, 1);
    }
    // pub fn check_core_configurator_for_events_block_type() {}
    // pub fn check_core_configurator_for_stream_block_type() {}
}
