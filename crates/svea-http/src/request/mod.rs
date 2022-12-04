use crate::path::{parse_as_path, Path};
use crate::{parse_body, BodyValue, Method};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Request {
    pub body: BodyValue,
    pub method: Method,
    pub path: Path,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub ip_address: Option<String>,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            body: BodyValue::Empty,
            method: Method::GET,
            path: "/".into(),
            headers: HashMap::new(),
            cookies: HashMap::new(),
            ip_address: None,
        }
    }
}

impl Request {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn path<P>(mut self, path: P) -> Self
    where
        P: Into<Path>,
    {
        self.path = path.into();
        self
    }

    pub fn body(mut self, body: BodyValue) -> Self {
        self.body = body;
        self
    }
}

impl ToString for Request {
    fn to_string(&self) -> String {
        let mut request = String::new();

        request.push_str(&format!(
            "{} {} HTTP/1.1\n",
            self.method.to_string(),
            self.path.to_string()
        ));

        for (key, value) in &self.headers {
            request.push_str(&format!("{}: {}\n", key, value));
        }

        request.push_str("\n");

        request.push_str(&self.body.to_string());

        request
    }
}

impl TryInto<Request> for String {
    type Error = String;

    fn try_into(self) -> Result<Request, Self::Error> {
        let mut lines = self.lines();

        let first_line = lines.next().unwrap();
        let mut parts = first_line.split_whitespace();

        let method = parts.next().unwrap().to_string();
        let method: Method = method.try_into().unwrap();
        let path = parts.next().unwrap().to_string();

        let mut headers = HashMap::new();
        let mut cookies = HashMap::new();

        for line in lines.clone() {
            let mut parts = line.splitn(2, ": ");

            let key = parts.next().unwrap().trim_start().to_string();

            if key == "Cookie" {
                for cookie in parts.next().unwrap().split("; ") {
                    let mut parts = cookie.splitn(2, "=");

                    cookies.insert(
                        parts.next().unwrap().to_string(),
                        parts.next().unwrap().to_string(),
                    );
                }
            }

            let value = match parts.next() {
                Some(i) => i.to_string(),
                None => continue,
            }
            .to_string();

            headers.insert(key, value);

            if line.is_empty() {
                break;
            }
        }

        let body: String = lines.skip_while(|line| !line.is_empty()).skip(1).collect();
        // ?
        let body = body.trim().to_string();

        // TODO: Understand why this happens?
        // Currently the body gets filled with \0\0\0\0.... many times...
        // TCP Magic or just poor parsing?
        let body = body.replace("\0", "");

        let body = parse_body(body, headers.get("Content-Type")).unwrap();

        let path = parse_as_path(path);

        Ok(Request {
            body,
            method,
            path,
            headers,
            cookies,
            /// Don't know what this is, is only known on the tcp stream.
            ip_address: None,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{BodyValue, Method};

    use super::Request;

    #[test]
    fn can_parse_request() {
        let request = "GET / HTTP/1.1
                            Host: localhost:8080
                            User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0
                            Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8
                            Accept-Language: en-US,en;q=0.5
                            Accept-Encoding: gzip, deflate
                            Connection: keep-alive
                            Upgrade-Insecure-Requests: 1
                            Cache-Control: max-age=0
                            Cookie: test=123

                            Hello, World!".to_string();

        let parsed: Request = request.try_into().unwrap();

        assert_eq!(parsed.method, Method::GET);
        assert_eq!(
            parsed.body,
            BodyValue::String(String::from("Hello, World!"))
        );
        assert_eq!(parsed.headers.get("Host").unwrap(), "localhost:8080");
        assert_eq!(parsed.cookies.get("test").unwrap(), "123")
    }
}
