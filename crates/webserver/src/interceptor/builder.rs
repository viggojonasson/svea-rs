use crate::{interceptor::Interceptor, request::Request, response::Response};
use futures::Future;

pub struct InterceptorBuilder {
    interceptor: Interceptor,
}

impl Into<Interceptor> for InterceptorBuilder {
    fn into(self) -> Interceptor {
        self.build()
    }
}

impl InterceptorBuilder {
    pub fn new() -> Self {
        Self {
            interceptor: Interceptor {
                activate_on: Box::new(|_| Box::pin(async { false })),
                on_request: Box::new(|_, res| Box::pin(async { res })),
                name: String::new(),
            },
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.interceptor.name = name;

        self
    }

    pub fn activate_on<F, Fut>(mut self, activate_on: F) -> Self
    where
        F: Fn(Request) -> Fut + 'static + Send + Sync,
        Fut: Future<Output = bool> + 'static + Send + Sync,
    {
        self.interceptor.activate_on = Box::new(move |r| Box::pin(activate_on(r)));

        self
    }

    pub fn on_request<F, Fut>(mut self, on_request: F) -> Self
    where
        F: Fn(Request, Response) -> Fut + 'static + Send + Sync,
        Fut: Future<Output = Response> + 'static + Send + Sync,
    {
        self.interceptor.on_request = Box::new(move |req, res| Box::pin(on_request(req, res)));

        self
    }

    pub fn build(self) -> Interceptor {
        self.interceptor
    }
}
