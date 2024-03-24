use serde::{Deserialize, Serialize};

use config::{Config, File};
use once_cell::sync::Lazy;

mod modules;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoreConfig {
    pub first_name: String,
    pub last_name: String,
}

// TODO: Take it up as a challenge
// impl <T> TryInto <T> for PathBuf<T> { }

pub static CONFIG: Lazy<CoreConfig> = Lazy::new(|| {
    const partial_path: &str = "conf/config.json";

    let mut abs_path = std::env::current_dir().unwrap();
    abs_path.push(partial_path);

    let full_path = match abs_path.to_str() {
        Some(p) => p,
        None => panic!("Something wrong with the path"),
    };

    let builder = Config::builder()
        .add_source(File::with_name(full_path))
        .build()
        .unwrap();

    builder.try_deserialize().unwrap()
});

impl CoreConfig {
    // pub fn load() -> Lazy<CoreConfig> {
    //     pub static CONFIG: Lazy<CoreConfig> = Lazy::new(|| {
    //         const partial_path: &str = "conf/config.json";

    //         let mut abs_path = std::env::current_dir().unwrap();
    //         abs_path.push(partial_path);

    //         let full_path = match abs_path.to_str() {
    //             Some(p) => p,
    //             None => panic!("Something wrong with the path"),
    //         };

    //         let builder = Config::builder()
    //             .add_source(File::with_name(full_path))
    //             .build()
    //             .unwrap();

    //         builder.try_deserialize().unwrap()
    //     });

    //     // return CONFIG;
    // }
    //
    //

    // pub fn parse_config(&self) -> bool {}
}
