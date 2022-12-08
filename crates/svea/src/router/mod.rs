use crate::router::route::Route;
use svea_http::Request;

pub mod route;

// TODO: Add base path to the router.
#[derive(Default)]
pub struct Router {
    pub routes: Vec<Route>,
    /// A base path to add to all the routes in this router.
    pub base_path: Option<String>,
}

impl Router {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base_path<P>(mut self, path: P) -> Self
    where
        P: Into<String>,
    {
        self.base_path = Some(path.into());
        self
    }

    /// Add a route to this router.
    pub fn route<R>(mut self, route: R) -> Self
    where
        R: Into<Route>,
    {
        self.routes.push(route.into());
        self
    }

    pub fn find_matching_route(&self, request: &Request) -> Option<&Route> {
        let mut request = request.clone();

        // Check if the base_path is present, if it is then check if the first
        // part of the path matches the base_path.
        if let Some(base_path) = &self.base_path {
            if !request.path.path.starts_with(base_path) {
                return None;
            }

            request.path.path = request.path.path.replace(base_path, "");
        }

        for route in &self.routes {
            if route.filter.matches_request(&request) {
                return Some(route);
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    fn create_request() -> Request {
        let path = Path::new()
            .path("/users/query")
            .query("user_id", QueryValue::Number(1.0));
        Request::new().path(path).method(Method::GET)
    }

    #[test]
    fn base_path_works() {
        let request = create_request();

        let query_route = Route::new()
            .path("/query")
            .handler(|_, _| async move { "[]" });

        let router = Router::new().base_path("/users").route(query_route);

        let route = router.find_matching_route(&request);

        assert_eq!(route.is_some(), true);
        assert_eq!(route.unwrap().filter.path, String::from("/query"))
    }
}
