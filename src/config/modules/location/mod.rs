// this module will house the various stages that can be used in the config files
// it will be stored in a hierarchial fashion
// eg > location block can proxy_pass, url rewriting, redirecting, caching, ratelimiting, error_page, auth_basic, add_header

mod proxy_pass;
mod rewrite;

pub enum Stage {
    Rewrite,
    AddHeader, // add a header
    ProxyPass, // for reverse proxy
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
