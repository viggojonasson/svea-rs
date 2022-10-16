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

pub struct UserDB(Vec<(String, String)>);

#[tokio::main]
async fn main() {
    Server::builder()
        .address("localhost".to_string())
        .port(3000)
        .state(UserDB {
            0: vec![("John".to_string(), "Doe".to_string())],
        })
        .router(
            Router::builder()
                .route(Route::builder().path("/").handler(|_, _| async move {
                    Response::builder()
                        .status(Status::Ok)
                        .body("<h1>Hello, world!</h1>")
                        .build()
                }))
                .route(
                    Route::builder()
                        .path("/users")
                        .handler(|server, _| async move {
                            let db = server.get_state::<UserDB>().unwrap();

                            let mut body = String::new();

                            for (first_name, last_name) in &db.0 {
                                body.push_str(&format!("{} {}<br>", first_name, last_name));
                            }

                            Response::builder().status(Status::Ok).body(body).build()
                        }),
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
        .build()
        .run()
        .await;
}
