use crate::{interceptor::builder::InterceptorBuilder, request::Request, response::Response};
use futures::future::BoxFuture;

pub mod builder;

/// TODO: Make this only need a reference to a request and a response.
/// TODO: Make the on_request get a mutable response instead of it having to return a new one.
pub struct Interceptor {
    pub activate_on: Option<Box<dyn Fn(Request) -> BoxFuture<'static, bool> + Sync + Send>>,
    pub on_request: Box<dyn Fn(Request, Response) -> BoxFuture<'static, Response> + Sync + Send>,
    pub activate_on_all: bool,
    pub name: String,
}

impl Interceptor {
    pub fn builder() -> InterceptorBuilder {
        InterceptorBuilder::new()
    }

    pub fn can_activate(&self, request: Request) -> BoxFuture<'static, bool> {
        if self.activate_on_all {
            return Box::pin(async { true });
        }

        if self.activate_on.is_some() {
            return self.activate_on.as_ref().unwrap()(request);
        }

        Box::pin(async { false })
    }
}
