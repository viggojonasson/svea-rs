use crate::server::Server;
use std::sync::Arc;
use svea_filter::Filter;
use svea_http::{Request, Response};

/// A service that runs a function after a request has been processed by a handler.
pub enum Service {
    Global(Box<dyn GlobalService + Send + Sync>),
    Filtered(Box<dyn FilteredService + Send + Sync>),
}

/// A global service that is ran on **every single** request, no exceptions.
pub trait GlobalService {
    fn on_request(&self, server: Arc<Server>, request: &Request, response: &mut Response);
}

/// A filtered service that is ran on every request that **matches the filter**.
pub trait FilteredService {
    fn on_request(&self, server: Arc<Server>, request: &Request, response: &mut Response);
    fn filter(&self) -> Filter;
}

/// Hello world example of use of a service.
pub struct HelloWorldService;

impl GlobalService for HelloWorldService {
    fn on_request(&self, _server: Arc<Server>, _request: &Request, response: &mut Response) {
        response.body += &"<br><h1>Hello World!</h1>";
        response.body += &format!("Address: {}", _server.address);
    }
}
