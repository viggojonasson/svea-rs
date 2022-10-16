use crate::{path::Path, router::route::Route};

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
    pub fn route(mut self, route: Route) -> Self {
        self.routes.push(route);
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
