//  this module is all about collecting and
// manipulating the incoming request before sending it
// forward to the converned destination server

mod request;
pub use request::CoreRequest;

pub trait Request {
    fn transform_request(stream: std::net::TcpStream) -> Vec<String>;

    // this wont create a new entity; it will
    // just add the required headers and return it back
    fn add_headers(response_array: Vec<String>, header_string: Vec<String>) -> Vec<String>;
}
