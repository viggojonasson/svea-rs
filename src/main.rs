use webserver::{
    http::{QueryValue, Status},
    interceptor::Interceptor,
    path::{queries::Queries, Path},
    request::Request,
    response::Response,
    server::Server,
};

async fn change_status(_request: Request, response: Response) -> Response {
    let mut r = response;

    r.body.push_str(" this request was intercepted");

    r.status = Status::ImATeapot;

    r
}

async fn change_body(_request: Request, response: Response) -> Response {
    let mut r = response;

    r.body
        .push_str(&format!("<h1>The new status code is {:#?}</h1>", r.status));

    r
}

async fn activate_on(request: Request) -> bool {
    if let Some(intercept) = request.path.queries.get_by_key("intercept".to_string()) {
        if intercept == &QueryValue::F32(1.0) {
            return true;
        }
    }

    false
}

#[tokio::main]
async fn main() {
    let change_status_interceptor = Interceptor::builder()
        .on_request(change_status)
        .activate_on(activate_on)
        .name("change_status");

    let add_status_to_body_interceptor = Interceptor::builder()
        .on_request(change_body)
        .activate_on_all(true)
        .name("add_status_to_body");

    let path = Path::new("/test".to_string(), Queries::new());

    Server::new("localhost".to_string(), 3000)
        .route(path, |req| {
            let mut queries = String::from("<h1>You sent no queries!</h1>");

            for (key, value) in req.path.queries.0 {
                if queries == "<h1>You sent no queries!</h1>" {
                    queries = String::from("");
                }
                queries.push_str(&format!("{}: {:#?}, ", key, value));
            }

            format!("<h1>Queries you sent</h1>{}", queries).into()
        })
        .interceptor(change_status_interceptor)
        .interceptor(add_status_to_body_interceptor)
        .fallback(|_request| {
            Response::builder()
                .status(Status::NotFound)
                .body("<h1>Page you tried to access does not exist!</h1>")
                .build()
        })
        .run()
        .await;
}
