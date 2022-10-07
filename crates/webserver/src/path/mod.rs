use crate::path::queries::Queries;
use std::collections::HashMap;
use webserver_http::Query;

pub mod queries;

#[derive(PartialEq)]
pub struct Path {
    pub path: String,
    pub queries: Queries,
}

impl Path {
    pub fn new(path: String, queries: Queries) -> Self {
        Self { path, queries }
    }
}

impl From<String> for Path {
    fn from(path: String) -> Self {
        let mut queries = Queries::new();
        let mut path = path;
        if let Some(index) = path.find('?') {
            let query = path.split_off(index + 1);

            let key = query.split('=').next().unwrap();
            let value = query.split('=').last().unwrap();
            queries.insert(key.to_string(), value.to_string().try_into().unwrap());
            path = path.split_off(index);
        }

        Self { path, queries }
    }
}
