use crate::Status;
use std::collections::hash_map::HashMap;

#[derive(Debug, Clone)]
pub struct Response {
    pub body: String,
    pub status: Status,
    pub headers: HashMap<String, String>,
}

impl TryInto<Response> for String {
    type Error = String;

    fn try_into(self) -> Result<Response, Self::Error> {
        let mut lines = self.lines();
        let mut status = Status::Ok;
        let mut headers = HashMap::new();
        let mut body = String::new();

        if let Some(line) = lines.next() {
            let mut parts = line.split_whitespace();

            if let Some(status_code) = parts.nth(1) {
                if let Ok(status_code) = status_code.parse::<u16>() {
                    status = Status::try_from(status_code).unwrap();
                }
            }
        }

        for line in lines.clone() {
            if line.is_empty() {
                break;
            }

            let mut parts = line.splitn(2, ':');

            if let Some(key) = parts.next() {
                if let Some(value) = parts.next() {
                    headers.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
        }

        for line in lines {
            body.push_str(&line.replace("\n", ""));
        }

        Ok(Response {
            body,
            status,
            headers,
        })
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}

impl Response {
    pub fn new() -> Self {
        Self {
            body: String::new(),
            status: Status::Ok,
            headers: HashMap::new(),
        }
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    pub fn body<T>(mut self, body: T) -> Self
    where
        T: Into<String>,
    {
        self.body = body.into();
        self
    }

    pub fn header<T>(mut self, key: T, value: T) -> Self
    where
        T: Into<String>,
    {
        self.headers.insert(key.into(), value.into());
        self
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
