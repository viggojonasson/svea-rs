use crate::router::{route::Route, Router};

pub struct RouterBuilder {
    router: Router,
}

impl Into<Router> for RouterBuilder {
    fn into(self) -> Router {
        self.build()
    }
}

impl RouterBuilder {
    pub fn new() -> RouterBuilder {
        RouterBuilder {
            router: Router::default(),
        }
    }

    pub fn route<R>(mut self, route: R) -> Self
    where
        R: Into<Route>,
    {
        self.router.routes.push(route.into());
        self
    }

    pub fn build(self) -> Router {
        self.router
    }
}
