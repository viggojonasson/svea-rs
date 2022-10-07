use webserver::{
    http::Status, interceptor::Interceptor, request::Request, response::Response, server::Server,
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

    Server::new("localhost".to_string(), 3000)
        .route("/", |_request| {
            "hello i hate siffran nio".to_string().into()
        })
        .interceptor(interceptor)
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
