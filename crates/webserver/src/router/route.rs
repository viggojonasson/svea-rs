use crate::{path::Path, request::Request, response::Response, server::Server};
use futures::{future::BoxFuture, Future};
use std::sync::Arc;

pub struct Route {
    pub path: Path,
    pub handler: Box<dyn Fn(Arc<Server>, Request) -> BoxFuture<'static, Response> + Sync + Send>,
}

impl Route {
    pub fn new<F, Fut, P>(path: P, handler: F) -> Self
    where
        F: Fn(Arc<Server>, Request) -> Fut + 'static + Send + Sync,
        Fut: Future<Output = Response> + 'static + Send + Sync,
        P: Into<Path>,
    {
        Self {
            path: path.into(),
            handler: Box::new(move |s, r| Box::pin(handler(s, r))),
        }
    }
}
