use crate::handler::Handler;
use crate::server::Server;
use std::future::Future;
use std::sync::Arc;
use webserver_http::{Path, Request, Response};

pub struct Route {
    pub path: Path,
    pub handler: Handler,
}

impl Route {
    /// Create a new route with an empty handler
    /// Unless handler is given this will panic when being ran.
    pub fn new() -> Self {
        async fn handler(_: Arc<Server>, _: Request) -> Response {
            panic!("RouteBuilder::new() was called without setting a handler");
        }

        Self {
            path: Path::builder().path("/").build(),
            handler: Handler::new(handler),
        }
    }

    pub fn path<P>(mut self, path: P) -> Self
    where
        P: Into<Path>,
    {
        self.path = path.into();
        self
    }

    pub fn handler<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(Arc<Server>, Request) -> Fut + 'static + Send + Sync,
        Fut: Future<Output = Response> + 'static + Send + Sync,
    {
        self.handler = Handler::new(handler);

        self
    }
}
