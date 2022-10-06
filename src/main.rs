use webserver::{http::status::Status, response::Response, server::Server};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    Server::new("localhost".to_string(), 3000)
        .route("/", |_request| {
            "hello i hate siffran nio".to_string().into()
        })
        .route("/hello", |request| {
            Response::builder()
                .status(Status::NotFound)
                .body(format!("hello {}", request.path))
                .build()
        })
        .fallback(|_request| {
            Response::builder()
                .status(Status::NotFound)
                .body("<h1>Page you tried to access does not exist!</h1>")
                .build()
        })
        .run()
        .await;
}
