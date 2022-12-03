use crate::handler::Handler;
use crate::server::Server;
use std::future::Future;
use std::sync::Arc;
use svea_filter::Filter;
use svea_http::{IntoResponse, Request};

pub struct Route {
    pub filter: Filter,
    pub handler: Handler,
}

impl Route {
    /// Create a new route with an empty handler
    /// Unless a handler is set, this route will print a warning message and just return a message saying that no handler was set
    pub fn new() -> Self {
        Self {
            filter: Filter::new("".to_string()),
            handler: Handler::default(),
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

    /// Set the handler for the route.
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
