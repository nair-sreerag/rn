use crate::config::structs::ConfigStruct;
use crate::config::traits::IConfig;
use crate::utils::Utils;

struct ConfigLoader {}

impl IConfig for ConfigLoader {
    fn load_config(path: String, contents: String) {
        let x = Utils::read_file(&path);

        let return_type: ConfigStruct = Utils::convert_to_json(x.as_str());
    }
}
