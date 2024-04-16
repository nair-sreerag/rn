// WHAT IS IT??
// this function will validate if the data provided in the config
// meets the operational requirements or not...
// This should NOT make any changes to the incoming request

mod validator;

use crate::config::CoreConfig;

pub trait CoreValidator {
    fn validate(config: &CoreConfig) -> bool;
}
