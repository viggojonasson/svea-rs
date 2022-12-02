use svea::{
    filter::{BodyFilter, Filter, QueryFilter},
    http::{Response, Status},
    router::{route::Route, Router},
    server::Server,
};

pub fn get_server(port: u16) -> Server {
    Server::new()
        .address("localhost".to_string())
        .port(port)
        .state(UserDB {
            0: vec![("John".to_string(), "Doe".to_string())],
        })
        .router(
            Router::new()
                .route(
                    Route::new()
                        .filter(
                            Filter::new("/")
                                .query("pi", QueryFilter::NumberExact(3.14))
                                .body(BodyFilter::StringExact("Hello!".to_string())),
                        )
                        .handler(|_, _| async move {
                            Response::new()
                                .status(Status::Ok)
                                .body("<h1>Hello, world!</h1>")
                        }),
                )
                .route(
                    Route::new()
                        .filter(Filter::new("/users"))
                        .handler(|server, _| async move {
                            let db = server.get_state::<UserDB>().unwrap();

                            let mut body = String::new();

                            for (first_name, last_name) in &db.0 {
                                body.push_str(&format!("{} {}<br>", first_name, last_name));
                            }

                            Response::new()
                                .status(Status::Ok)
                                .body(body)
                                .header("User-Amount", &format!("{}", db.0.len()))
                        }),
                )
                .route(
                    Route::new()
                        .filter(Filter::new("/into-response"))
                        .handler(|_, _| async move { "Hello, world!" }),
                ),
        )
        .fallback(|_, _| async move {
            Response::new()
                .status(Status::NotFound)
                .body("<h1>Page you tried to access does not exist!</h1>")
        })
}

pub struct UserDB(Vec<(String, String)>);

#[tokio::main]
async fn main() {
    get_server(3000).run().await;
}

#[cfg(test)]
mod tests {
    use super::get_server;
    use svea::http::{BodyValue, Request, Status};
    use svea_client::Client;
    use tokio::test;

    #[test]
    async fn test_get_users() {
        get_server(3000).spawn().await;

        let mut client = Client::builder().address("localhost").port(3000).build();

        let res = client.send(Request::new().path("/users")).await.unwrap();

        assert_eq!(res.status, Status::Ok);
        assert_eq!(res.headers.get("User-Amount").unwrap(), "1");
    }

    #[test]
    async fn test_not_found() {
        get_server(3001).spawn().await;

        let mut client = Client::builder().address("localhost").port(3001).build();

        let res = client
            .send(Request::new().path("/not-found"))
            .await
            .unwrap();

        assert_eq!(res.status, Status::NotFound);
        assert_eq!(
            res.body,
            "<h1>Page you tried to access does not exist!</h1>"
        );
    }

    #[test]
    async fn test_index() {
        get_server(3002).spawn().await;

        let mut client = Client::builder().address("localhost").port(3002).build();

        let res = client
            .send(
                Request::new()
                    .path("/?pi=3.14")
                    .body(BodyValue::String("Hello!".to_string())),
            )
            .await
            .unwrap();

        assert_eq!(res.status, Status::Ok);
        assert_eq!(res.body, "<h1>Hello, world!</h1>");
    }
}
