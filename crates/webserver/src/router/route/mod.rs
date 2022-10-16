use crate::{
    path::Path, request::Request, response::Response, router::route::builder::RouteBuilder,
    server::Server,
};
use futures::future::BoxFuture;
use std::sync::Arc;

pub mod builder;

pub struct Route {
    pub path: Path,
    pub handler: Box<dyn Fn(Arc<Server>, Request) -> BoxFuture<'static, Response> + Sync + Send>,
}

impl Route {
    pub fn builder() -> RouteBuilder {
        RouteBuilder::new()
    }
}
