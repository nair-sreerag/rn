mod validator;

use crate::config::CoreConfig;

pub trait CoreValidator {
    // this function will validate if the data provided in the config
    // meets the operational requirements or not
    //
    fn validate(config: &CoreConfig) -> bool;
}
