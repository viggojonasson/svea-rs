use crate::handler::Handler;
use crate::server::Server;
use std::future::Future;
use std::sync::Arc;
use webserver_filter::Filter;
use webserver_http::{IntoResponse, Request};

pub struct Route {
    pub filter: Filter,
    pub handler: Handler,
}

impl Route {
    /// Create a new route with an empty handler
    /// Unless handler is given this will panic when being ran.
    pub fn new() -> Self {
        async fn handler(_: Arc<Server>, _: Request) -> String {
            String::from("No handler was given for this route")
        }

        Self {
            filter: Filter::new("".to_string()),
            handler: Handler::new(handler),
        }
    }

    /// Shorthand for setting the path for the filter.
    pub fn path<P>(mut self, path: P) -> Self
    where
        P: Into<String>,
    {
        self.filter.path = path.into();
        self
    }

    /// Add a filter to the route.
    pub fn filter<P>(mut self, filter: P) -> Self
    where
        P: Into<Filter>,
    {
        self.filter = filter.into();
        self
    }

    pub fn handler<F, Fut, R>(mut self, handler: F) -> Self
    where
        F: Fn(Arc<Server>, Request) -> Fut + 'static + Send + Sync,
        Fut: Future<Output = R> + 'static + Send + Sync,
        R: IntoResponse + 'static,
    {
        self.handler = Handler::new(handler);

        self
    }
}
