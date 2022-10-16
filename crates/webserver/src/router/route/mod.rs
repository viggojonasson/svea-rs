use crate::{handler::Handler, path::Path, router::route::builder::RouteBuilder};

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
