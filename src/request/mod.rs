use std::collections::HashMap;

pub struct Request {
    pub body: String,
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
}

impl TryInto<Request> for String {
    type Error = String;

    fn try_into(self) -> Result<Request, Self::Error> {
        let mut lines = self.lines();

        let first_line = lines.next().unwrap();
        let mut parts = first_line.split_whitespace();

        let method = parts.next().unwrap().to_string();
        let path = parts.next().unwrap().to_string();

        let mut headers = HashMap::new();

        for line in lines.clone() {
            let mut parts = line.splitn(2, ": ");
            let key = parts.next().unwrap().to_string();
            let value = parts.next().unwrap().to_string();

            headers.insert(key, value);

            if line.is_empty() {
                break;
            }
        }

        let body = lines.collect::<Vec<&str>>().join("\r");

        Ok(Request {
            body,
            method,
            path,
            headers,
        })
    }
}
