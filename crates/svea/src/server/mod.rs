use crate::router::route::Route;
use crate::service::{FilteredService, GlobalService, Service};
use crate::{handler::Handler, router::Router};
use std::future::Future;
use std::{any::Any, sync::Arc};
use svea_http::{IntoResponse, Path, Request};
use tokio::net::TcpListener;

pub mod connection;

pub struct Server {
    pub address: String,
    pub port: u16,
    pub routers: Vec<Router>,
    /// A fallback is a handler that gets executed whenever we get a request that is un-routable
    pub fallback: Option<Handler>,
    pub states: Vec<Arc<dyn std::any::Any + Send + Sync>>,
    pub path: Option<Path>,
    pub services: Vec<Service>,
    /// Whether to respond to requests that would map to 404.
    pub no_default_404: bool,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            address: "localhost".to_string(),
            port: 3000,
            fallback: None,
            states: Vec::new(),
            path: None,
            routers: vec![],
            services: vec![],
            no_default_404: false,
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether to respond to requests that would map to 404.
    pub fn no_default_404(mut self, no_default_404: bool) -> Self {
        self.no_default_404 = no_default_404;
        self
    }

    /// Attach a global service
    /// TODO: Make all service attach methods into one
    pub fn global_service<P>(mut self, service: P) -> Self
    where
        P: GlobalService + Send + Sync + 'static,
    {
        self.services.push(Service::Global(Box::new(service)));
        self
    }

    /// Attach a fileted service
    /// TODO: Make all service attach methods into one
    pub fn filtered_service<P>(mut self, service: P) -> Self
    where
        P: FilteredService + Send + Sync + 'static,
    {
        self.services.push(Service::Filtered(Box::new(service)));
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Add a router to this server.
    /// TODO: Make the router pass an Path.
    pub fn router<R>(mut self, router: R) -> Self
    where
        R: Into<Router>,
    {
        self.routers.push(router.into());
        self
    }

    /// Add a route, this will create a completely new router to add this route to.
    pub fn route<R>(mut self, route: R) -> Self
    where
        R: Into<Route>,
    {
        let router = Router::new().route(route.into());

        self.routers.push(router);
        return self;
    }

    /// Sets the fallback handler for the server.
    pub fn fallback<F, Fut, R>(mut self, fallback: F) -> Self
    where
        F: Fn(Arc<Server>, Request) -> Fut + 'static + Send + Sync,
        Fut: Future<Output = R> + 'static + Send + Sync,
        R: IntoResponse + 'static,
    {
        self.fallback = Some(Handler::new(fallback));
        self
    }

    /// Set the state of the server.
    pub fn state<T: 'static + Send + Sync>(mut self, state: T) -> Self {
        self.states.push(Arc::new(state));

        self
    }

    /// Set the address of the server.
    pub fn address<T>(mut self, address: T) -> Self
    where
        T: Into<String>,
    {
        self.address = address.into();
        self
    }

    /// Get server state by type.
    pub fn get_state<T: Any + Send + Sync>(&self) -> Option<&T> {
        for state in &self.states {
            if let Some(state) = state.downcast_ref::<T>() {
                return Some(state);
            }
        }

        None
    }

    /// Spawn a new task to run the server for you.
    pub async fn spawn(self) {
        self.run_server(true).await
    }

    /// Run the server.
    pub async fn run(self) {
        self.run_server(false).await
    }

    async fn run_server(self, spawn: bool) {
        let server = Arc::new(self);

        println!("Listening on {}:{}", server.address, server.port);

        let listener = TcpListener::bind(format!("{}:{}", server.address, server.port))
            .await
            .unwrap();

        let runner = || async move {
            loop {
                let (mut stream, _) = listener.accept().await.unwrap();

                let server = Arc::clone(&server);
                tokio::spawn(async move {
                    connection::handle_connection(&mut stream, server).await;
                });
            }
        };

        if spawn {
            tokio::spawn(async move { runner().await });
        } else {
            runner().await
        }
    }
}
