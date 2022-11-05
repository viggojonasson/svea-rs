use crate::router::route::Route;
use webserver_http::Path;

pub mod route;

pub struct Router {
    pub routes: Vec<Route>,
}

impl Default for Router {
    fn default() -> Self {
        Self { routes: vec![] }
    }
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

    pub fn find_matching_route(&self, path: &Path) -> Option<&Route> {
        for route in &self.routes {
            if route.path.queries.0.len() == 0 {
                if route.path.path == path.path {
                    return Some(route);
                }
            }

            if &route.path == path {
                return Some(route);
            }
        }

        None
    }
}
