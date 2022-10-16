use crate::{
    handler::Handler, interceptor::Interceptor, path::Path, request::Request, response::Response,
    router::Router,
};
use futures::Future;
use std::sync::Arc;
use tokio::net::TcpListener;

pub mod connection;

pub struct Server {
    pub address: String,
    pub port: u16,
    pub router: Router,
    pub fallback: Option<Handler>,
    pub interceptors: Vec<Interceptor>,
}

impl Server {
    pub fn new(address: String, port: u16) -> Self {
        Self {
            address,
            port,
            fallback: None,
            interceptors: Vec::new(),
            router: Router::default(),
        }
    }

    /// Set an router for this server.
    /// TODO: Make the router pass an Path and allow for multiple routers.
    pub fn router<R>(mut self, router: R) -> Self
    where
        R: Into<Router>,
    {
        self.router = router.into();
        self
    }

    /// Sets the fallback handler for the server.
    pub fn fallback<F, Fut>(mut self, fallback: F) -> Self
    where
        F: Fn(Arc<Server>, Request) -> Fut + 'static + Send + Sync,
        Fut: Future<Output = Response> + 'static + Send + Sync,
    {
        self.fallback = Some(Handler::new(fallback));
        self
    }

    pub fn interceptor(mut self, interceptor: impl Into<Interceptor>) -> Self {
        self.interceptors.push(interceptor.into());
        self
    }

    pub async fn run(self) {
        let server = Arc::new(self);

        println!("Listening on {}:{}", server.address, server.port);

        let listener = TcpListener::bind(format!("{}:{}", server.address, server.port))
            .await
            .unwrap();

        loop {
            let (mut stream, _) = listener.accept().await.unwrap();

            let server = Arc::clone(&server);
            tokio::spawn(async move {
                connection::handle_connection(&mut stream, server).await;
            });
        }
    }
}
