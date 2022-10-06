use crate::{http::status::Status, request::Request, response::Response, server::Server};
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn handle_connection(stream: &mut TcpStream, server: Arc<Server>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let request: Request = String::from_utf8_lossy(&buffer)
        .to_string()
        .try_into()
        .unwrap();

    let response = map_request(request, server).await;

    stream
        .write(String::from(response).as_bytes())
        .await
        .unwrap();
}

pub async fn map_request(request: Request, server: Arc<Server>) -> Response {
    match server.routes.get(&request.path) {
        Some(handler) => handler(request),
        None => Response::builder()
            .status(Status::NotFound)
            .body("Not Found")
            .build(),
    }
}
