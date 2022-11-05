use crate::{Response, Status};

pub trait IntoResponse {
    fn into_response(&self) -> Response;
}

impl IntoResponse for String {
    fn into_response(&self) -> Response {
        Response::new().body(self)
    }
}

impl IntoResponse for Response {
    fn into_response(&self) -> Response {
        self.clone()
    }
}

impl IntoResponse for &str {
    fn into_response(&self) -> Response {
        Response::new().body(*self)
    }
}

impl IntoResponse for Status {
    fn into_response(&self) -> Response {
        Response::new().status(self.clone())
    }
}

impl IntoResponse for (&str, Status) {
    fn into_response(&self) -> Response {
        Response::new().status(self.1.clone()).body(self.0)
    }
}
