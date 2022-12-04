use svea_http::{BodyValue, Method, Path, QueryValue, Request};

#[derive(Default)]
pub struct Filter {
    pub path: String,
    pub queries: Vec<(String, QueryFilter)>,
    pub body: Option<BodyFilter>,
    pub methods: Vec<Method>,
    pub headers: Vec<(String, Option<String>)>,
}

impl Filter {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            queries: Vec::new(),
            ..Default::default()
        }
    }

    /// Filter a request by a header and value.
    /// If the value is None then the filter will only filter the header being present.
    pub fn header(mut self, key: impl Into<String>, value: Option<String>) -> Self {
        self.headers.push((key.into(), value));
        self
    }

    /// Filter a request by this method.
    pub fn method(mut self, method: Method) -> Self {
        if self.methods.contains(&method) {
            return self;
        }

        self.methods.push(method);
        self
    }

    /// Filter a request by a body filter.
    pub fn body(mut self, body: BodyFilter) -> Self {
        self.body = Some(body);
        self
    }

    /// Filter a request by a query filter.
    pub fn query(mut self, name: impl Into<String>, filter: QueryFilter) -> Self {
        self.queries.push((name.into(), filter));
        self
    }

    /// Check if the request matches the filter.
    pub fn matches_request(&self, request: &Request) -> bool {
        let method = &request.method;

        // If we have no methods we assume that we want to match to all methods.
        if self.methods.len() != 0 && !self.methods.contains(method) {
            return false;
        }

        let path = &request.path;
        let body = &request.body;

        // Check first if the path matches.
        if !self.matches_path(&path) {
            return false;
        }

        // If it does, check if the body matches.
        if let Some(body_filter) = &self.body {
            if !body_filter.cmp_body_value(&body) {
                return false;
            }
        }

        for (needed_key, needed_value) in &self.headers {
            let mut satisfies = false;

            for (given_key, given_value) in &request.headers {
                if needed_key == given_key {
                    if let Some(value) = needed_value {
                        if given_value == value {
                            satisfies = true;
                        }
                    } else {
                        satisfies = true;
                    }
                }
            }

            if !satisfies {
                println!("{needed_key} does not satisfy!");
                return false;
            }
        }

        true
    }

    /// Check if the path matches the filter. (Does not check body (obviously))
    pub fn matches_path(&self, path: &Path) -> bool {
        if self.path != path.path {
            return false;
        }

        let mut needs_to_satisfy = self.queries.len();

        for (key, filter) in &self.queries {
            if let Some(value) = path.queries.get_by_key(key.clone()) {
                if filter.cmp_query_value(value) {
                    needs_to_satisfy -= 1;
                }
            }
        }

        needs_to_satisfy == 0
    }
}

pub enum QueryFilter {
    String,
    Number,
    Bool,
    StringExact(String),
    NumberExact(f64),
    BoolExact(bool),
}

impl QueryFilter {
    fn cmp_query_value(&self, value: &QueryValue) -> bool {
        match value {
            QueryValue::Bool(b1) => match self {
                QueryFilter::Bool => true,
                QueryFilter::BoolExact(b2) => b1 == b2,
                _ => false,
            },
            QueryValue::String(s1) => match self {
                QueryFilter::StringExact(s2) => s1 == s2,
                QueryFilter::String => true,
                _ => false,
            },
            QueryValue::Number(n1) => match self {
                QueryFilter::NumberExact(n2) => n1 == n2,
                QueryFilter::Number => true,
                _ => false,
            },
        }
    }
}

impl PartialEq for QueryFilter {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (QueryFilter::String, QueryFilter::String) => true,
            (QueryFilter::Number, QueryFilter::Number) => true,
            (QueryFilter::Bool, QueryFilter::Bool) => true,
            (QueryFilter::StringExact(a), QueryFilter::StringExact(b)) => a == b,
            (QueryFilter::NumberExact(a), QueryFilter::NumberExact(b)) => a == b,
            (QueryFilter::BoolExact(a), QueryFilter::BoolExact(b)) => a == b,
            _ => false,
        }
    }
}

pub enum BodyFilter {
    String,
    StringExact(String),
}

impl BodyFilter {
    fn cmp_body_value(&self, other: &BodyValue) -> bool {
        match other {
            BodyValue::String(s1) => match self {
                BodyFilter::StringExact(s2) => s1 == s2,
                BodyFilter::String => true,
            },
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Filter;
    use crate::QueryFilter;
    use svea_http::{Method, Path, QueryValue, Request};

    fn create_request() -> Request {
        let path = Path::new()
            .query("Test-Query", QueryValue::Number(420.0))
            .path("/");
        Request::new()
            .header("Test-Header", "Value")
            .method(Method::GET)
            .path(path)
    }

    #[test]
    fn test_header_filter() {
        let req = create_request();
        let filter_exact = Filter::new("/").header("Test-Header", Some(String::from("Value")));
        let filter_any = Filter::new("/").header("Test-Header", None);
        let filter_wrong = Filter::new("/").header("Test-Header", Some(String::from("Not Value")));

        assert_eq!(filter_exact.matches_request(&req), true);
        assert_eq!(filter_any.matches_request(&req), true);
        assert_eq!(filter_wrong.matches_request(&req), false);
    }

    #[test]
    fn test_query_filter() {
        let req = create_request();
        let filter_correct = Filter::new("/").query("Test-Query", QueryFilter::NumberExact(420.0));
        let filter_incorrect =
            Filter::new("/").query("Test-Query", QueryFilter::NumberExact(420.69));

        assert_eq!(filter_correct.matches_request(&req), true);
        assert_eq!(filter_incorrect.matches_request(&req), false);
    }

    #[test]
    fn test_method_filter() {
        let req = create_request();
        let filter_correct = Filter::new("/").method(Method::GET);
        let filter_incorrect = Filter::new("/").method(Method::DELETE);
        // Any method will match to this filter.
        let filter_any = Filter::new("/");
        assert_eq!(filter_correct.matches_request(&req), true);
        assert_eq!(filter_incorrect.matches_request(&req), false);
        assert_eq!(filter_any.matches_request(&req), true);
    }
}
