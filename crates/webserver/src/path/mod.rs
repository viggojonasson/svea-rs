use crate::path::queries::Queries;

pub mod queries;

#[derive(PartialEq, Clone)]
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
    fn from(mut input: String) -> Self {
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

        Self { path, queries }
    }
}