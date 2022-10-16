use std::sync::Arc;

use webserver::{
    http::{QueryValue, Status},
    interceptor::Interceptor,
    path::Path,
    request::Request,
    response::Response,
    router::{route::Route, Router},
    server::Server,
};

async fn append_query(_req: Request, res: Response) -> Response {
    let mut r = res;

    r.body.push_str(
        "
<br>
<h1>Intercepted!</h1>
        ",
    );

    r
}

#[tokio::main]
async fn main() {
    Server::new("localhost".to_string(), 3000)
        .router(
            Router::builder().route(
                Route::builder()
                    .path(Path::builder().path("/").build())
                    .handler(|_, _| async move {
                        Response::builder()
                            .status(Status::Ok)
                            .body("<h1>Hello, world!</h1>")
                            .build()
                    })
                    .build(),
            ),
        )
        .interceptor(
            Interceptor::builder()
                .name("append query")
                .on_request(append_query)
                .activate_on_all(true),
        )
        .fallback(|_, _| async move {
            Response::builder()
                .status(Status::NotFound)
                .body("<h1>Page you tried to access does not exist!</h1>")
                .build()
        })
        .run()
        .await;
}
