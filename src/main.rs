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
                .status(Status::Ok)
                .body(format!(
                    "hello you are doing a {} request to {}, peer address is: {}",
                    request.method.to_string(),
                    request.path,
                    request.ip_address.unwrap()
                ))
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
