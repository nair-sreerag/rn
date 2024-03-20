mod types;
pub use types::*;

pub trait Server {
    // initializes with all the required things
    fn new() -> Self;

    // starts the server
    fn start();

    //  used to check if the total permissible number of the threads
    // exceeds the total system capacity
    fn get_permissible_limit() -> u32;
}
