use crate::{server::Server, service::Service};
use std::sync::Arc;
use svea_http::{Request, Response, Status};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn handle_connection(stream: &mut TcpStream, server: Arc<Server>) {
    let mut buffer = [0; 1024];
    let _read = stream.read(&mut buffer).await.unwrap();

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

    if let Ok(addr) = stream.peer_addr() {
        request.ip_address = Some(addr.ip().to_string());
    }

    let response = map_request(request.clone(), server.clone()).await;

    if response.is_none() {
        return;
    }

    let mut response = response.unwrap();

    for service in &server.services {
        match service {
            Service::Global(service) => {
                service
                    .on_request(server.clone(), &request, &mut response)
                    .await;
            }
            Service::Filtered(service) => {
                if service.filter().matches_request(&request) {
                    service
                        .on_request(server.clone(), &request, &mut response)
                        .await;
                }
            }
        }
    }

    let _written = stream
        .write(String::from(response).as_bytes())
        .await
        .unwrap();

    stream.flush().await.unwrap();
}

pub async fn map_request(request: Request, server: Arc<Server>) -> Option<Response> {
    // Go through all our routers and see which router can handle this request.
    // TODO?: Maybe cache this?
    for router in &server.routers {
        match router.find_matching_route(&request) {
            Some(route) => {
                return Some(route.handler.handle(server.clone(), request).await);
            }
            None => {}
        }
    }

    // Check if we have a fallback that can handle this request.
    match &server.fallback {
        Some(fallback) => Some(fallback.handle(server.clone(), request).await),
        None => {
            if server.no_default_404 == true {
                None
            } else {
                Some(Response::new().status(Status::NotFound))
            }
        }
    }
}
