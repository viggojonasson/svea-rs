use crate::{Method, Path, Request};

pub struct RequestBuilder {
    request: Request,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self {
            request: Request::default(),
        }
    }

    pub fn method(mut self, method: Method) -> Self {
        self.request.method = method;
        self
    }

    pub fn path<P>(mut self, path: P) -> Self
    where
        P: Into<Path>,
    {
        self.request.path = path.into();
        self
    }

    pub fn body(mut self, body: String) -> Self {
        self.request.body = body;
        self
    }

    pub fn build(self) -> Request {
        self.request
    }
}
