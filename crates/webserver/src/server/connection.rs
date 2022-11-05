use crate::server::Server;
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use webserver_http::{Request, Response, Status};

pub async fn handle_connection(stream: &mut TcpStream, server: Arc<Server>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let mut request: Request = match String::from_utf8_lossy(&buffer).to_string().try_into() {
        Ok(request) => request,
        Err(_) => {
            let response = Response::new().status(Status::BadRequest);
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

    let mut response = map_request(request.clone(), server.clone()).await;

    for interceptor in &server.interceptors {
        if interceptor.can_activate(request.clone()).await {
            println!("Using interceptor {}", interceptor.name);
            response = (interceptor.on_request)(request.clone(), response.clone()).await;
        }
    }

    stream
        .write(String::from(response).as_bytes())
        .await
        .unwrap();

    stream.flush().await.unwrap();
}

pub async fn map_request(request: Request, server: Arc<Server>) -> Response {
    match server.router.find_matching_route(&request.path) {
        Some(route) => route.handler.handle(server.clone(), request).await,
        None => match &server.fallback {
            Some(fallback) => fallback.handle(server.clone(), request).await,
            None => Response::new().status(Status::NotFound),
        },
    }
}
