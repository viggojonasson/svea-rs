use crate::{interceptor::builder::InterceptorBuilder, request::Request, response::Response};
use futures::future::BoxFuture;

pub mod builder;

/// TODO: Make this only need a reference to a request and a response.
/// TODO: Make the on_request get a mutable response instead of it having to return a new one.
pub struct Interceptor {
    pub activate_on: Box<dyn Fn(Request) -> BoxFuture<'static, bool> + Sync + Send>,
    pub on_request: Box<dyn Fn(Request, Response) -> BoxFuture<'static, Response> + Sync + Send>,
    pub name: String,
}

impl Interceptor {
    pub fn builder() -> InterceptorBuilder {
        InterceptorBuilder::new()
    }

    pub fn can_activate(&self, request: Request) -> BoxFuture<'static, bool> {
        (self.activate_on)(request)
    }
}
