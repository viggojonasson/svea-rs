use crate::{request::Request, response::Response};
use std::{collections::HashMap, sync::Arc};
use tokio::net::TcpListener;

pub mod connection;

pub struct Server {
    pub address: String,
    pub port: u16,
    pub routes: HashMap<String, Box<dyn Fn(Request) -> Response + Send + Sync>>,
}

impl Server {
    pub fn new(address: String, port: u16) -> Self {
        Self {
            address,
            port,
            routes: HashMap::new(),
        }
    }

    pub fn route(
        &mut self,
        path: &str,
        handler: impl Fn(Request) -> Response + Send + Sync + 'static,
    ) {
        self.routes.insert(path.to_string(), Box::new(handler));
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
