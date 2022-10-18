use crate::path::{queries::Queries, Path};
use crate::QueryValue;

impl Into<Path> for PathBuilder {
    fn into(self) -> Path {
        self.build()
    }
}

pub struct PathBuilder {
    path: Path,
}

impl PathBuilder {
    pub fn new() -> Self {
        Self {
            path: Path::new(String::new(), Queries::new()),
        }
    }

    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path.path = path.into();
        self
    }

    pub fn query(mut self, key: impl Into<String>, value: QueryValue) -> Self {
        self.path.queries.insert(key.into(), value);
        self
    }

    pub fn queries(mut self, queries: Queries) -> Self {
        self.path.queries = queries;
        self
    }

    pub fn build(self) -> Path {
        self.path
    }
}
