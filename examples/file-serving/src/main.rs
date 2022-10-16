use std::sync::Arc;

use tokio::fs::read_to_string;
use webserver::{request::Request, response::Response, server::Server};

async fn read_from_fs(_server: Arc<Server>, request: Request) -> Response {
    let path = request.path.path;

    let file = read_to_string(format!(
        "{}{}",
        "./examples/file-serving/static",
        match path == "/" {
            true => "/index.html",
            false => &path,
        }
    ))
    .await
    .unwrap();

    Response::builder().body(file).build()
}

#[tokio::main]
async fn main() {
    // We can use fallback to serve static files.
    // NOTE: Only works if we have no additional routes that would clash with the file path.
    Server::new("localhost".to_string(), 3000)
        .fallback(read_from_fs)
        .run()
        .await;
}
