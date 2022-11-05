use webserver::{
    http::Status,
    router::{route::Route, Router},
    server::Server,
};

#[tokio::main]
async fn main() {
    Server::new()
        .address("localhost".to_string())
        .port(3000)
        .router(
            Router::new().route(
                Route::new()
                    .path("/")
                    .handler(|_, _| async move { ("Hello, World!", Status::ImATeapot) }),
            ),
        )
        .run()
        .await;
}
