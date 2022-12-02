use crate::{path::queries::Queries, QueryValue};
pub mod queries;

#[derive(PartialEq, Clone)]
pub struct Path {
    pub path: String,
    pub queries: Queries,
}

impl Path {
    pub fn new() -> Self {
        Self {
            path: String::new(),
            queries: Queries::new(),
        }
    }

    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }

    pub fn query(mut self, key: impl Into<String>, value: QueryValue) -> Self {
        self.queries.insert(key.into(), value);
        self
    }

    pub fn queries(mut self, queries: Queries) -> Self {
        self.queries = queries;
        self
    }
}

impl ToString for Path {
    fn to_string(&self) -> String {
        let mut path = self.path.clone();

        if !self.queries.0.is_empty() {
            path.push('?');

            for (key, value) in &self.queries.0 {
                if let Some(val) = value.get_value_as_string() {
                    path.push_str(&format!("{}={}&", key, val));
                } else {
                    path.push_str(&format!("{}&", key));
                }
            }

            // Remove the last '&' character
            path.pop();
        }

        path
    }
}

impl Into<Path> for &str {
    fn into(self) -> Path {
        Path::new().path(self)
    }
}

pub fn parse_as_path(mut input: String) -> Path {
    let mut queries = Queries::new();
    let mut path = input.clone();

    if let Some(index) = input.find('?') {
        let query = input.split_off(index + 1);

        for query in query.split('&') {
            let mut query = query.split('=');
            let key = query.next().unwrap().to_string();
            let value = query.next().unwrap().to_string();
            queries.insert(key, value.into());
        }
    }

    // Remove everything behind ? in the path string.
    if let Some(index) = path.find('?') {
        let _ = path.split_off(index);
    }

    Path { path, queries }
}
