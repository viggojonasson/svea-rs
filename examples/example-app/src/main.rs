use webserver::{
    http::{QueryValue, Status},
    interceptor::Interceptor,
    path::Path,
    request::Request,
    response::Response,
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
        .route(
            Path::builder()
                .path("/test")
                .query("query", QueryValue::Bool(Some(false))),
            |_req| "<h1>This is a test path</h1>".to_string().into(),
        )
        .interceptor(
            Interceptor::builder()
                .name("append query")
                .on_request(append_query)
                .activate_on_all(true),
        )
        .fallback(|_request| {
            Response::builder()
                .status(Status::NotFound)
                .body("<h1>Page you tried to access does not exist!</h1>")
                .build()
        })
        .run()
        .await;
}
