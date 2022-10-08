use webserver::{
    http::Status,
    interceptor::Interceptor,
    path::{queries::Queries, Path},
    request::Request,
    response::Response,
    server::Server,
};

async fn intercept(request: Request, response: Response) -> Response {
    let mut r = response;

    r.body.push_str(" (intercepted)");

    r
}

async fn activate_on(request: Request) -> bool {
    true
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let interceptor = Interceptor {
        activate_on: Box::new(move |r| Box::pin(activate_on(r))),
        on_request: Box::new(move |req, res| Box::pin(intercept(req, res))),
    };

    let path = Path::new("/test".to_string(), Queries::new());

    Server::new("localhost".to_string(), 3000)
        .route(path, |req| {
            let mut queries = String::new();

            for (key, value) in req.path.queries.0 {
                queries.push_str(&format!("{}: {:#?}, ", key, value));
            }

            format!("<h1>Queries you sent</h1><br>{}", queries).into()
        })
        .interceptor(interceptor)
        .fallback(|_request| {
            Response::builder()
                .status(Status::NotFound)
                .body("<h1>Page you tried to access does not exist!</h1>")
                .build()
        })
        .run()
        .await;
}
