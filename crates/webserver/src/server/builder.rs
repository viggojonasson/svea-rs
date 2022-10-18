use crate::{handler::Handler, interceptor::Interceptor, router::Router, server::Server};
use futures::Future;
use std::sync::Arc;
use webserver_http::{Request, Response};

pub struct ServerBuilder {
    server: Server,
}
impl Into<Server> for ServerBuilder {
    fn into(self) -> Server {
        self.build()
    }
}

impl ServerBuilder {
    pub fn new() -> ServerBuilder {
        ServerBuilder {
            server: Server::default(),
        }
    }

    pub fn address(mut self, address: String) -> ServerBuilder {
        self.server.address = address;
        self
    }

    pub fn port(mut self, port: u16) -> ServerBuilder {
        self.server.port = port;
        self
    }

    pub fn state<T: 'static + Send + Sync>(mut self, state: T) -> Self {
        self.server.states.push(Arc::new(state));

        self
    }

    /// Set an router for this server.
    /// TODO: Make the router pass an Path and allow for multiple routers.
    pub fn router<R>(mut self, router: R) -> Self
    where
        R: Into<Router>,
    {
        self.server.router = router.into();
        self
    }

    /// Sets the fallback handler for the server.
    pub fn fallback<F, Fut>(mut self, fallback: F) -> Self
    where
        F: Fn(Arc<Server>, Request) -> Fut + 'static + Send + Sync,
        Fut: Future<Output = Response> + 'static + Send + Sync,
    {
        self.server.fallback = Some(Handler::new(fallback));
        self
    }

    pub fn interceptor(mut self, interceptor: impl Into<Interceptor>) -> Self {
        self.server.interceptors.push(interceptor.into());
        self
    }

    pub fn build(self) -> Server {
        self.server
    }
}
