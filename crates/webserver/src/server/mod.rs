use crate::{handler::Handler, interceptor::Interceptor, router::Router};
use std::future::Future;
use std::{any::Any, sync::Arc};
use tokio::net::TcpListener;
use webserver_http::{Path, Request, Response};

pub mod connection;

pub struct Server {
    pub address: String,
    pub port: u16,
    pub router: Router,
    pub fallback: Option<Handler>,
    pub interceptors: Vec<Interceptor>,
    pub states: Vec<Arc<dyn std::any::Any + Send + Sync>>,
    pub path: Option<Path>,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            address: "localhost".to_string(),
            port: 3000,
            router: Router::default(),
            fallback: None,
            interceptors: Vec::new(),
            states: Vec::new(),
            path: None,
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
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

    /// Add an interceptor.
    pub fn interceptor(mut self, interceptor: impl Into<Interceptor>) -> Self {
        self.interceptors.push(interceptor.into());
        self
    }

    /// Set the state of the server.
    pub fn state<T: 'static + Send + Sync>(mut self, state: T) -> Self {
        self.states.push(Arc::new(state));

        self
    }

    /// Set the address of the server.
    pub fn address<T>(mut self, address: T) -> Self
    where
        T: Into<String>,
    {
        self.address = address.into();
        self
    }

    /// Get server state by type.
    pub fn get_state<T: Any + Send + Sync>(&self) -> Option<&T> {
        for state in &self.states {
            if let Some(state) = state.downcast_ref::<T>() {
                return Some(state);
            }
        }

        None
    }

    /// Spawn a new task to run the server for you.
    pub async fn spawn(self) {
        self.run_server(true).await
    }

    /// Run the server.
    pub async fn run(self) {
        self.run_server(false).await
    }

    async fn run_server(self, spawn: bool) {
        let server = Arc::new(self);

        println!("Listening on {}:{}", server.address, server.port);

        let listener = TcpListener::bind(format!("{}:{}", server.address, server.port))
            .await
            .unwrap();

        let runner = || async move {
            loop {
                let (mut stream, _) = listener.accept().await.unwrap();

                let server = Arc::clone(&server);
                tokio::spawn(async move {
                    connection::handle_connection(&mut stream, server).await;
                });
            }
        };

        if spawn {
            tokio::spawn(async move { runner().await });
        } else {
            runner().await
        }
    }
}
