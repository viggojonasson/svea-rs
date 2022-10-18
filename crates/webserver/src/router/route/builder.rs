use crate::{handler::Handler, router::Route, server::Server};
use core::panic;
use futures::Future;
use std::sync::Arc;
use webserver_http::{Path, Request, Response};

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
                handler: Handler::new(handler),
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
        self.route.handler = Handler::new(handler);

        self
    }

    pub fn build(self) -> Route {
        self.route
    }
}
