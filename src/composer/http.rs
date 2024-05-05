use crate::block::ServerLocationBlock;
use crate::composer::Composer;
use crate::config::CoreConfig;
use std::net::TcpStream;

use crate::config::cluster::ClusterConfigurationComposition;
use crate::config::location::LocationConfigurationComposition;
use crate::config::server_http::ServerHttpLayout;
use crate::config::server_https::ServerHttpsLayout;
use crate::config::BlockType;

/// the load balancer composer
pub struct HttpLoadBalancerComposer {
    https_block: ServerHttpsLayout,
    http_block: ServerHttpLayout,
    cluster_block: Vec<ClusterConfigurationComposition>,
    location_block: Vec<LocationConfigurationComposition>,
}

impl HttpLoadBalancerComposer {
    fn new(
        https_block: ServerHttpsLayout,
        http_block: ServerHttpLayout,
        cluster_block: Vec<ClusterConfigurationComposition>,
        location_block: Vec<LocationConfigurationComposition>,
    ) -> Self {
        HttpLoadBalancerComposer {
            http_block,
            https_block,
            cluster_block,
            location_block,
        }
    }
}

impl Composer for HttpLoadBalancerComposer {
    fn serve_request(
        stream: TcpStream,
        config: CoreConfig,
        location_blocks: ServerLocationBlock,
        response_object: bool,
    ) -> () {
        todo!()
    }
}
