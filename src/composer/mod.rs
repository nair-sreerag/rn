

mod http;


/// WHAT DOES IT DO?
/// Pulls all the required components together and
/// serves the request
pub trait Composer {
    // fn new() -> Self;

    fn serve_request(
        stream: std::net::TcpStream,
        config: crate::config::CoreConfig,
        location_blocks: crate::block::ServerLocationBlock,
        // cluster_blocks: crate::block::,
        response_object: bool,
    ) -> bool;
}
