// WHAT IS IT??
// this module will check if the passed config file has all the
// required keys or not. Does NOT check the validity of the data

use std::env::current_dir;

use serde::{Deserialize, Serialize};

use config::{Config, ConfigError, File};
use once_cell::sync::Lazy;

mod layout;

pub use layout::BLOCK_TYPE;

use self::layout::{
    cluster::ClusterConfigurationComposition, location::LocationConfigurationComposition,
    ConfigRootLevelComposition,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoreConfig {
    pub configs: Vec<ConfigRootLevelComposition>,
}

// TODO: Take it up as a challenge
// impl <T> TryInto <T> for PathBuf<T> { }

impl CoreConfig {
    pub fn load() -> Result<Self, ConfigError> {
        // there can be multiple json files in this folder
        // each json file which points to a specific service maybe
        let inner_path = "conf/config.json";
        let mut abs_path = current_dir().unwrap();

        abs_path.push(inner_path);

        let config_builder = Config::builder()
            .add_source(File::with_name(&abs_path.to_string_lossy()))
            .build()
            .unwrap();

        let z = config_builder.try_deserialize();

        println!("zz m {:?}", z);

        z
    }

    pub fn get(self) -> Self {
        self
    }
}

pub static CONFIG: Lazy<CoreConfig> = Lazy::new(|| {
    // TODO: check this
    let inner_path = "conf/config.json";
    let mut abs_path = current_dir().unwrap();

    abs_path.push(inner_path);

    let config_builder = Config::builder()
        .add_source(File::with_name(&abs_path.to_string_lossy()))
        .build()
        .unwrap();

    let deserialized_config = config_builder.try_deserialize();

    println!("zz m {:?}", deserialized_config);

    deserialized_config.unwrap()
});

// pub static CONFIG: Lazy<CoreConfig> = Lazy::new(|| {
//     const partial_path: &str = "conf/config.json";

//     let mut abs_path = std::env::current_dir().unwrap();
//     abs_path.push(partial_path);

//     let full_path = match abs_path.to_str() {
//         Some(p) => p,
//         None => panic!("Something wrong with the path"),
//     };

//     let builder = Config::builder()
//         .add_source(File::with_name(full_path))
//         .build()
//         .unwrap();

//     builder.try_deserialize().unwrap()
// });
