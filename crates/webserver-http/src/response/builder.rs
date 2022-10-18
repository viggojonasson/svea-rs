use crate::response::Response;
use crate::Status;

pub struct ResponseBuilder {
    response: Response,
}

impl Into<Response> for ResponseBuilder {
    fn into(self) -> Response {
        self.build()
    }
}

impl ResponseBuilder {
    pub fn new() -> Self {
        Self {
            response: Response::default(),
        }
    }

    pub fn status(mut self, status: Status) -> Self {
        self.response.status = status;
        self
    }

    pub fn body<T>(mut self, body: T) -> Self
    where
        T: Into<String>,
    {
        self.response.body = body.into();
        self
    }

    pub fn header<T>(mut self, key: T, value: T) -> Self
    where
        T: Into<String>,
    {
        self.response.headers.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> Response {
        self.response
    }
}
