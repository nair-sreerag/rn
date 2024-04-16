// this module will house the various stages that can be used in the config files
// it will be stored in a hierarchial fashion
// eg > location block can proxy_pass, url rewriting, redirecting, caching, ratelimiting, error_page, auth_basic, add_header

use serde::{Deserialize, Serialize};

use self::rewrite::RewriteStage;

mod proxy_pass;
mod rewrite;

mod add_header;

#[derive(Debug, Clone, Serialize)]
pub enum Stage {
    Rewrite {
        // original_string: String,
        grouping_regex: String,
        replacement_regex: String,
        should_redirect: bool,
    },
    AddHeader, // add a header
    ProxyPass {
        url: String, // for reverse proxy
    },
    AddAuthHeader,
    ModifyHeader,
    LimitConnections,
    ProxyCache,
    StaticFile,
    //
    //
    // FastCGI
}

pub trait LocationStage {
    type StageInitArgs;

    fn init(args: Self::StageInitArgs) -> Self;

    fn process(self) -> String;
}
