use webserver::{http::status::Status, response::Response, server::Server};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut server = Server::new("localhost".to_string(), 3000);

    server.route("/", |_request| {
        "hello i hate siffran nio".to_string().into()
    });

    server.fallback(|_request| {
        Response::builder()
            .status(Status::NotFound)
            .body("<h1>Page you tried to access does not exist!</h1>")
            .build()
    });

    server.route("/hello", |request| {
        Response::builder()
            .status(Status::NotFound)
            .body(format!("hello {}", request.path))
            .build()
    });

    server.run().await;
}
