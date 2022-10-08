use webserver_http::QueryValue;

#[derive(PartialEq, Clone)]
pub struct Queries(pub Vec<(String, QueryValue)>);

impl Queries {
    pub fn new() -> Self {
        Queries(Vec::new())
    }

    pub fn insert(&mut self, key: String, value: QueryValue) {
        self.0.push((key, value));
    }

    pub fn remove_by_key(&mut self, key: String) -> bool {
        todo!();
    }
}
