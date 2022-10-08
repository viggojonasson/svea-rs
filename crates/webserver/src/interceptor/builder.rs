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
                activate_on: None,
                on_request: Box::new(|_, res| Box::pin(async { res })),
                name: String::new(),
                activate_on_all: false,
            },
        }
    }

    /// Internal name for the interceptor.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.interceptor.name = name.into();

        self
    }

    /// If this is true the interceptor will run on all requests.
    /// By default it only runs if the self.activate_on function returns true.
    pub fn activate_on_all(mut self, run_on_all: bool) -> Self {
        self.interceptor.activate_on_all = run_on_all;

        self
    }

    /// Function that returns true if the interceptor should run on the request.
    /// If this is None the interceptor will only run if self.run_on_all is true.
    pub fn activate_on<F, Fut>(mut self, activate_on: F) -> Self
    where
        F: Fn(Request) -> Fut + 'static + Send + Sync,
        Fut: Future<Output = bool> + 'static + Send + Sync,
    {
        self.interceptor.activate_on = Some(Box::new(move |r| Box::pin(activate_on(r))));

        self
    }

    /// Function that will run on the request.
    /// This function will only run if self.activate_on returns true.
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
