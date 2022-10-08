use crate::{request::Request, response::Response, server::Server};
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use webserver_http::Status;

pub async fn handle_connection(stream: &mut TcpStream, server: Arc<Server>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let mut request: Request = match String::from_utf8_lossy(&buffer).to_string().try_into() {
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

    match stream.peer_addr() {
        Ok(addr) => request.ip_address = Some(addr.ip().to_string()),
        Err(_) => {}
    }

    let response = map_request(request.clone(), server.clone()).await;

    let interceptor = server.interceptors.first().unwrap();

    let response = (interceptor.on_request)(request, response).await;

    stream
        .write(String::from(response).as_bytes())
        .await
        .unwrap();

    stream.flush().await.unwrap();
}

pub async fn map_request(request: Request, server: Arc<Server>) -> Response {
    match server.routes.find_matching_handler(&request.path) {
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
