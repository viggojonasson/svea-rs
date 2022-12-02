use webserver_http::{Path, QueryValue};

pub struct Filter {
    pub path: String,
    pub queries: Vec<(String, QueryFilter)>,
    pub body: Option<QueryFilter>,
}

impl Filter {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            queries: Vec::new(),
            body: None,
        }
    }

    pub fn query(mut self, name: impl Into<String>, filter: QueryFilter) -> Self {
        self.queries.push((name.into(), filter));
        self
    }

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
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
}

impl QueryFilter {
    fn cmp_query_value(&self, value: &QueryValue) -> bool {
        match value {
            QueryValue::Bool(_) => {
                if self == &QueryFilter::Bool {
                    true
                } else {
                    false
                }
            }
            QueryValue::String(_) => {
                if self == &QueryFilter::String {
                    true
                } else {
                    false
                }
            }
            QueryValue::U8(_) => {
                if self == &QueryFilter::U8 {
                    true
                } else {
                    false
                }
            }
            QueryValue::U16(_) => {
                if self == &QueryFilter::U16 {
                    true
                } else {
                    false
                }
            }
            QueryValue::U32(_) => {
                if self == &QueryFilter::U32 {
                    true
                } else {
                    false
                }
            }
            QueryValue::U64(_) => {
                if self == &QueryFilter::U64 {
                    true
                } else {
                    false
                }
            }
            QueryValue::I8(_) => {
                if self == &QueryFilter::I8 {
                    true
                } else {
                    false
                }
            }
            QueryValue::I16(_) => {
                if self == &QueryFilter::I16 {
                    true
                } else {
                    false
                }
            }
            QueryValue::I32(_) => {
                if self == &QueryFilter::I32 {
                    true
                } else {
                    false
                }
            }
            QueryValue::I64(_) => {
                if self == &QueryFilter::I64 {
                    true
                } else {
                    false
                }
            }
            QueryValue::F32(_) => {
                if self == &QueryFilter::F32 {
                    true
                } else {
                    false
                }
            }
            QueryValue::F64(_) => {
                if self == &QueryFilter::F64 {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

impl PartialEq for QueryFilter {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (QueryFilter::String, QueryFilter::String) => true,
            (QueryFilter::U8, QueryFilter::U8) => true,
            (QueryFilter::U16, QueryFilter::U16) => true,
            (QueryFilter::U32, QueryFilter::U32) => true,
            (QueryFilter::U64, QueryFilter::U64) => true,
            (QueryFilter::I8, QueryFilter::I8) => true,
            (QueryFilter::I16, QueryFilter::I16) => true,
            (QueryFilter::I32, QueryFilter::I32) => true,
            (QueryFilter::I64, QueryFilter::I64) => true,
            (QueryFilter::F32, QueryFilter::F32) => true,
            (QueryFilter::F64, QueryFilter::F64) => true,
            (QueryFilter::Bool, QueryFilter::Bool) => true,

            _ => false,
        }
    }
}

pub enum BodyFilter {
    String,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
}
