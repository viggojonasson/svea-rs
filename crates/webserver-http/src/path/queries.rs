use crate::QueryValue;

#[derive(PartialEq, Clone)]
pub struct Queries(pub Vec<(String, QueryValue)>);

impl Queries {
    pub fn new() -> Self {
        Queries(Vec::new())
    }

    pub fn insert(&mut self, key: String, value: QueryValue) {
        self.0.push((key, value.into()));
    }

    pub fn get_by_key(&self, key: String) -> Option<&QueryValue> {
        for (k, v) in &self.0 {
            if k == &key {
                return Some(v);
            }
        }

        None
    }

    pub fn get_by_value(&self, value: QueryValue) -> Option<&String> {
        for (k, v) in &self.0 {
            if v == &value {
                return Some(k);
            }
        }

        None
    }

    pub fn remove_by_key(&mut self, key: String) -> bool {
        todo!();
    }
}
