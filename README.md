# webserver-rs

Just playing around with Rust and web servers. Trying to learn rust.

### Features

- Blazingly slow
- Unoptimized

### Example Code:

```rs
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
```

## Run Examples:

- `file-serving`
- - `cargo run --bin file-serving`
- `example-app`
- - `cargo run --bin example-app`
- `readme-example`
- - `cargo run --bin readme-example`
