use super::{ConfigHolder, ConfigTraits};

impl ConfigTraits for ConfigHolder {
    fn check_if_port_is_available() -> bool {
        true
    }

    fn check_if_worker_threads_can_be_made() -> bool {
        true
    }
}
