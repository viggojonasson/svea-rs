use crate::{path::Path, request::Request, response::Response, router::Route, server::Server};
use core::panic;
use futures::Future;
use std::sync::Arc;

pub struct RouteBuilder {
    route: Route,
}

impl Into<Route> for RouteBuilder {
    fn into(self) -> Route {
        self.build()
    }
}

impl RouteBuilder {
    pub fn new() -> RouteBuilder {
        async fn handler(_: Arc<Server>, _: Request) -> Response {
            panic!("RouteBuilder::build() was called without setting a handler");
        }

        RouteBuilder {
            route: Route {
                path: Path::builder().path("/").build(),
                handler: Box::new(move |s, r| Box::pin(handler(s, r))),
            },
        }
    }

    pub fn path<P>(mut self, path: P) -> Self
    where
        P: Into<Path>,
    {
        self.route.path = path.into();
        self
    }

    pub fn handler<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(Arc<Server>, Request) -> Fut + 'static + Send + Sync,
        Fut: Future<Output = Response> + 'static + Send + Sync,
    {
        self.route.handler = Box::new(move |s, r| Box::pin(handler(s, r)));

        self
    }

    pub fn build(self) -> Route {
        self.route
    }
}
