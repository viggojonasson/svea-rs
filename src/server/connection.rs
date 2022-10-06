use crate::{http::status::Status, request::Request, response::Response, server::Server};
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn handle_connection(stream: &mut TcpStream, server: Arc<Server>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let request: Request = match String::from_utf8_lossy(&buffer).to_string().try_into() {
        Ok(request) => request,
        Err(_) => {
            let response = Response::builder().status(Status::BadRequest).build();
            stream
                .write_all(String::from(response).as_bytes())
                .await
                .unwrap();
            return;
        }
    };

    let response = map_request(request, server).await;

    stream
        .write(String::from(response).as_bytes())
        .await
        .unwrap();

    stream.flush().await.unwrap();
}

pub async fn map_request(request: Request, server: Arc<Server>) -> Response {
    match server.routes.get(&request.path) {
        Some(handler) => handler(request),
        None => {
            if let Some(fallback) = &server.fallback {
                fallback(request)
            } else {
                Response::builder()
                    .status(Status::NotFound)
                    .body("Not Found")
                    .build()
            }
        }
    }
}
