pub mod actual;
pub mod default;

pub struct ConfigHolder {
    port: u32,
    worker_size: u32,
}

trait ConfigTraits {
    fn check_if_port_is_available() -> bool;

    fn check_if_worker_threads_can_be_made() -> bool;
}
