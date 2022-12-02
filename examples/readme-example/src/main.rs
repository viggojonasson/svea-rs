use webserver::{
    filter::Filter,
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
                    .filter(Filter::new("/"))
                    .handler(|_, _| async move { ("Hello, World!", Status::ImATeapot) }),
            ),
        )
        .run()
        .await;
}
