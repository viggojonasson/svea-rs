use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use webserver_http::{Request, Response};

pub struct Client<'a> {
    port: u16,
    address: &'a str,
    stream: Option<TcpStream>,
}

impl<'a> Default for Client<'a> {
    fn default() -> Self {
        Self {
            address: "localhost",
            port: 3000,
            stream: None,
        }
    }
}

impl<'a> Client<'a> {
    pub fn builder() -> ClientBuilder<'a> {
        ClientBuilder::new()
    }

    /// Send a request to the server and return the response.
    pub async fn send(&mut self, request: Request) -> io::Result<Response> {
        if self.stream.is_none() {
            self.connect().await?;
        }

        let stream = self.stream.as_mut().unwrap();

        let request_as_str = request.to_string();

        stream.write_all(request_as_str.as_bytes()).await?;

        let mut response = String::new();

        stream.read_to_string(&mut response).await?;

        Ok(response.try_into().unwrap())
    }

    /// Try to connect to the server.
    pub async fn connect(&mut self) -> io::Result<()> {
        match TcpStream::connect(format!("{}:{}", self.address, self.port)).await {
            Ok(stream) => {
                self.stream = Some(stream);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

pub struct ClientBuilder<'a> {
    client: Client<'a>,
}

impl<'a> ClientBuilder<'a> {
    pub fn new() -> Self {
        Self {
            client: Client::default(),
        }
    }

    pub fn port(mut self, port: u16) -> Self {
        self.client.port = port;

        self
    }

    pub fn address<A>(mut self, address: A) -> Self
    where
        A: Into<&'a str>,
    {
        self.client.address = address.into();

        self
    }

    pub fn build(self) -> Client<'a> {
        self.client
    }
}

impl<'a> Into<Client<'a>> for ClientBuilder<'a> {
    fn into(self) -> Client<'a> {
        self.build()
    }
}
