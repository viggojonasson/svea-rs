use crate::router::route::Route;
use webserver_filter::Filter;
use webserver_http::Request;

pub mod route;

#[derive(Default)]
pub struct Router {
    pub routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn route<R>(mut self, route: R) -> Self
    where
        R: Into<Route>,
    {
        self.routes.push(route.into());
        self
    }

    pub fn find_matching_route(&self, request: &Request) -> Option<&Route> {
        for route in &self.routes {
            if request.path.path == route.filter.path {
                return Some(route);
            }
        }

        None
    }
}
