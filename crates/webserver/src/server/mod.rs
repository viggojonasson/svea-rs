use crate::{
    interceptor::Interceptor, path::Path, request::Request, response::Response, router::Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;

pub mod connection;

pub type Handler = Box<dyn Fn(Request) -> Response + Send + Sync>;

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

    pub fn router<R>(mut self, router: R) -> Self
    where
        R: Into<Router>,
    {
        self.router = router.into();
        self
    }

    pub fn fallback(
        mut self,
        handler: impl Fn(Request) -> Response + Send + Sync + 'static,
    ) -> Self {
        self.fallback = Some(Box::new(handler));
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
