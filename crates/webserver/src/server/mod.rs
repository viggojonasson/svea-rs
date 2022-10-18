use crate::{
    handler::Handler, interceptor::Interceptor, path::Path, request::Request, response::Response,
    router::Router,
};
use futures::Future;
use std::{any::Any, sync::Arc};
use tokio::net::TcpListener;

use self::builder::ServerBuilder;

pub mod builder;
pub mod connection;

pub struct Server {
    pub address: String,
    pub port: u16,
    pub router: Router,
    pub fallback: Option<Handler>,
    pub interceptors: Vec<Interceptor>,
    pub states: Vec<Arc<dyn std::any::Any + Send + Sync>>,
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
        }
    }
}

impl Server {
    pub fn builder() -> ServerBuilder {
        ServerBuilder::new()
    }

    pub fn get_state<T: Any + Send + Sync>(&self) -> Option<&T> {
        for state in &self.states {
            if let Some(state) = state.downcast_ref::<T>() {
                return Some(state);
            }
        }

        None
    }

    pub async fn run(self) {
        let server = Arc::new(self);

        println!("Listening on {}:{}", server.address, server.port);

        let listener = TcpListener::bind(format!("{}:{}", server.address, server.port))
            .await
            .unwrap();

        tokio::spawn(async move {
            loop {
                let (mut stream, _) = listener.accept().await.unwrap();

                let server = Arc::clone(&server);
                tokio::spawn(async move {
                    connection::handle_connection(&mut stream, server).await;
                });
            }
        });
    }
}
