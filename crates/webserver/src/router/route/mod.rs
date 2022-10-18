use crate::{handler::Handler, router::route::builder::RouteBuilder};
use webserver_http::Path;

pub mod builder;

pub struct Route {
    pub path: Path,
    pub handler: Handler,
}

impl Route {
    pub fn builder() -> RouteBuilder {
        RouteBuilder::new()
    }
}
