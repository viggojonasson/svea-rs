use std::sync::Arc;
use svea::{
    http::{IntoResponse, Request, Response, Status},
    server::Server,
};
use tokio::fs::read_to_string;

async fn read_from_fs(_server: Arc<Server>, request: Request) -> (String, Status) {
    let path = request.path.path;

    match read_to_string(format!(
        "{}{}",
        "./examples/file-serving/static",
        match &path == "/" {
            true => "/index.html",
            false => &path,
        }
    ))
    .await
    {
        Ok(contents) => (contents, Status::Ok),
        Err(e) => match e.kind() {
            tokio::io::ErrorKind::NotFound => (String::from("File not found!"), Status::NotFound),
            _ => (format!("Error: {}", e), Status::InternalServerError),
        },
    }
}

#[tokio::main]
async fn main() {
    // We can use fallback to serve static files.
    // NOTE: Only works if we have no additional routes that would clash with the file path.
    Server::new()
        .port(3000)
        .address("localhost".to_string())
        .fallback(read_from_fs)
        .run()
        .await;
}
