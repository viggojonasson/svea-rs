use crate::server::Server;
use futures::{future::BoxFuture, Future};
use std::sync::Arc;
use svea_http::{IntoResponse, Request, Response};

/// A function that handles a request and returns a response.
pub struct Handler(
    Box<dyn Fn(Arc<Server>, Request) -> BoxFuture<'static, Box<dyn IntoResponse>> + Sync + Send>,
);

impl Default for Handler {
    fn default() -> Self {
        async fn handler(_: Arc<Server>, _r: Request) -> String {
            println!("WARNING: No handler set for this route");
            String::from("No handler set for this route")
        }

        Self::new(handler)
    }
}

impl Handler {
    pub fn new<F, Fut, R>(handler: F) -> Self
    where
        F: Fn(Arc<Server>, Request) -> Fut + 'static + Send + Sync,
        Fut: Future<Output = R> + 'static + Send + Sync,
        R: IntoResponse + 'static,
    {
        let handler = move |server: Arc<Server>, request: Request| {
            let response = handler(server, request);
            let response = async move { Box::new(response.await) as Box<dyn IntoResponse> };
            Box::pin(response)
        };

        Handler(Box::new(move |s, r| Box::pin(handler(s, r))))
    }

    pub async fn handle(&self, server: Arc<Server>, request: Request) -> Response {
        (self.0)(server, request).await.into_response()
    }
}
