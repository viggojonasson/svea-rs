pub mod parsing;

#[derive(PartialEq)]
pub struct Filter {
    pub path: String,
    pub queries: Vec<(String, QueryType)>,
    pub body: Option<RequestBody>,
}

impl Filter {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            queries: Vec::new(),
            body: None,
        }
    }

    pub fn body(mut self, body: RequestBody) -> Self {
        self.body = Some(body);
        self
    }

    pub fn query(mut self, key: &str, query_type: QueryType) -> Self {
        self.queries.push((key.to_string(), query_type));
        self
    }

    pub fn queries(mut self, queries: Vec<(String, QueryType)>) -> Self {
        self.queries = queries;
        self
    }
}

#[derive(PartialEq, Debug)]
pub enum QueryType {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<QueryType>),
    Object(Vec<(String, QueryType)>),
}

#[derive(PartialEq, Debug)]
pub enum RequestBody {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<RequestBody>),
    Object(Vec<(String, RequestBody)>),
}
