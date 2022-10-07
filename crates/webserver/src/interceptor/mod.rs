use futures::future::BoxFuture;

use crate::{request::Request, response::Response};

pub struct Interceptor {
    pub activate_on: Box<dyn Fn(Request) -> BoxFuture<'static, bool> + Sync + Send>,
    pub on_request: Box<dyn Fn(Request, Response) -> BoxFuture<'static, Response> + Sync + Send>,
}
