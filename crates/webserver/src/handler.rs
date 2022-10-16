use crate::{request::Request, response::Response, server::Server};
use futures::{future::BoxFuture, Future};
use std::sync::Arc;

/// A function that handles a request and returns a response.
pub struct Handler(Box<dyn Fn(Arc<Server>, Request) -> BoxFuture<'static, Response> + Sync + Send>);

impl Handler {
    pub fn new<F, Fut>(handler: F) -> Self
    where
        F: Fn(Arc<Server>, Request) -> Fut + 'static + Send + Sync,
        Fut: Future<Output = Response> + 'static + Send + Sync,
    {
        Handler(Box::new(move |s, r| Box::pin(handler(s, r))))
    }

    pub async fn handle(&self, server: Arc<Server>, request: Request) -> Response {
        (self.0)(server, request).await
    }
}
