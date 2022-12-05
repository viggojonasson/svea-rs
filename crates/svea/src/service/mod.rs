use crate::server::Server;
use std::sync::Arc;
use svea_filter::Filter;
use svea_http::{Request, Response};

#[derive(PartialEq, Eq)]
pub enum ServiceType {
    /// Run this server after an appropriate handler has handled the request.
    Last,
}

/// A service is a function that is ran on every request that matches the filter.
pub trait Service {
    fn on_request(&self, server: Arc<Server>, request: &Request, response: &mut Response);
    fn service_type(&self) -> ServiceType;
    fn filter(&self) -> Filter;
}

/// Hello world example of use of a service.
pub struct HelloWorldService;

impl Service for HelloWorldService {
    fn on_request(&self, _server: Arc<Server>, _request: &Request, response: &mut Response) {
        response.body += &"<br><h1>Hello World!</h1>";
        response.body += &format!("Address: {}", _server.address);
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Last
    }

    fn filter(&self) -> Filter {
        Filter::new("/not-found")
    }
}
