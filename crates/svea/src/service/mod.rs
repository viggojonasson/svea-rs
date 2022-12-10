use crate::server::Server;
use async_trait::async_trait;
use std::sync::Arc;
use svea_filter::Filter;
use svea_http::{Request, Response};

/// A service that runs a function after a request has been processed by a handler.
pub enum Service {
    Global(Box<dyn GlobalService + Send + Sync>),
    Filtered(Box<dyn FilteredService + Send + Sync>),
}

/// A global service that is ran on **every single** request, no exceptions.
#[async_trait]
pub trait GlobalService {
    async fn on_request(&self, server: Arc<Server>, request: &Request, response: &mut Response);
}

/// A filtered service that is ran on every request that **matches the filter**.
#[async_trait]
pub trait FilteredService {
    async fn on_request(&self, server: Arc<Server>, request: &Request, response: &mut Response);
    fn filter(&self) -> Filter;
}
