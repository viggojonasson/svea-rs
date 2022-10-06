use webserver::{
    http::status::Status, request::Request, response::builder::ResponseBuilder, server::Server,
};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut server = Server::new("localhost".to_string(), 3000);

    server.route("/", |_request| {
        "hello i hate siffran nio".to_string().into()
    });

    server.route("/hello", |request| {
        ResponseBuilder::new()
            .status(Status::NotFound)
            .body("<h1>Här finns inget </h1>")
            .build()
    });

    server.run().await;
}
