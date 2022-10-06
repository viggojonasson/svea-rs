use crate::http::status::Status;
use std::collections::hash_map::HashMap;

pub mod builder;

#[derive(Debug)]
pub struct Response {
    pub body: String,
    pub status: Status,
    pub headers: HashMap<String, String>,
}

impl Into<Response> for String {
    fn into(self) -> Response {
        Response {
            body: self,
            status: Status::Ok,
            headers: HashMap::new(),
        }
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}

impl Response {
    fn new() -> Self {
        Self {
            body: String::new(),
            status: Status::Ok,
            headers: HashMap::new(),
        }
    }
}

impl From<Response> for String {
    fn from(response: Response) -> Self {
        let mut string = String::new();

        string.push_str(&format!("HTTP/1.1 {}\n", response.status.to_string()));
        for (key, value) in response.headers {
            string.push_str(&format!("{}: {}\n", key, value));
        }
        string.push_str("\n");
        string.push_str(&response.body);

        string
    }
}
