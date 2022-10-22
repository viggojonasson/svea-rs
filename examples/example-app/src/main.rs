use webserver::{
    http::{Response, Status},
    router::{route::Route, Router},
    server::Server,
};

pub fn get_server() -> Server {
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

                            Response::builder()
                                .status(Status::Ok)
                                .body(body)
                                .header("User-Amount", &format!("{}", db.0.len()))
                                .build()
                        }),
                ),
        )
        .fallback(|_, _| async move {
            Response::builder()
                .status(Status::NotFound)
                .body("<h1>Page you tried to access does not exist!</h1>")
                .build()
        })
        .build()
}

pub struct UserDB(Vec<(String, String)>);

#[tokio::main]
async fn main() {
    get_server().run().await;
}

#[cfg(test)]
mod tests {
    use super::get_server;
    use tokio::test;
    use webserver::http::{Request, Status};
    use webserver_client::Client;

    #[test]
    async fn test_get_users() {
        get_server().spawn().await;

        let mut client = Client::builder().address("localhost").port(3000).build();

        let res = client
            .send(Request::builder().path("/users").build())
            .await
            .unwrap();

        assert_eq!(res.status, Status::Ok);
        assert_eq!(res.headers.get("User-Amount").unwrap(), "1");
    }

    #[test]
    async fn test_not_found() {
        get_server().spawn().await;

        let mut client = Client::builder().address("localhost").port(3000).build();

        let res = client
            .send(Request::builder().path("/not-found").build())
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
        get_server().spawn().await;

        let mut client = Client::builder().address("localhost").port(3000).build();

        let res = client
            .send(Request::builder().path("/").build())
            .await
            .unwrap();

        assert_eq!(res.status, Status::Ok);
        assert_eq!(res.body, "<h1>Hello, world!</h1>");
    }
}
