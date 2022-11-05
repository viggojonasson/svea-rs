use futures::{future::BoxFuture, Future};
use webserver_http::{Request, Response};

/// TODO: Make this only need a reference to a request and a response.
/// TODO: Make the on_request get a mutable response instead of it having to return a new one.
pub struct Interceptor {
    pub activate_on: Option<Box<dyn Fn(Request) -> BoxFuture<'static, bool> + Sync + Send>>,
    pub on_request: Box<dyn Fn(Request, Response) -> BoxFuture<'static, Response> + Sync + Send>,
    pub activate_on_all: bool,
    pub name: String,
}

impl Interceptor {
    pub fn new() -> Self {
        Self {
            activate_on: None,
            on_request: Box::new(|_, res| Box::pin(async { res })),
            name: String::new(),
            activate_on_all: false,
        }
    }

    /// Internal name for the interceptor.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();

        self
    }

    /// If this is true the interceptor will run on all requests.
    /// By default it only runs if the self.activate_on function returns true.
    pub fn activate_on_all(mut self, run_on_all: bool) -> Self {
        self.activate_on_all = run_on_all;

        self
    }

    /// Function that returns true if the interceptor should run on the request.
    /// If this is None the interceptor will only run if self.run_on_all is true.
    pub fn activate_on<F, Fut>(mut self, activate_on: F) -> Self
    where
        F: Fn(Request) -> Fut + 'static + Send + Sync,
        Fut: Future<Output = bool> + 'static + Send + Sync,
    {
        self.activate_on = Some(Box::new(move |r| Box::pin(activate_on(r))));

        self
    }

    /// Function that will run on the request.
    /// This function will only run if self.activate_on returns true.
    pub fn on_request<F, Fut>(mut self, on_request: F) -> Self
    where
        F: Fn(Request, Response) -> Fut + 'static + Send + Sync,
        Fut: Future<Output = Response> + 'static + Send + Sync,
    {
        self.on_request = Box::new(move |req, res| Box::pin(on_request(req, res)));

        self
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
