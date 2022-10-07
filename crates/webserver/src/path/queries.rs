use webserver_http::Query;

#[derive(PartialEq)]
pub struct Queries(Vec<(String, Query)>);

impl Queries {
    pub fn new() -> Self {
        Queries(Vec::new())
    }

    pub fn insert(&mut self, key: String, value: Query) {
        self.0.push((key, value));
    }

    pub fn remove_by_key(&mut self, key: String) -> bool {
        todo!();
    }
}
