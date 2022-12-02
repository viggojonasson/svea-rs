#[derive(Clone, Debug)]
pub enum QueryValue {
    Number(f64),
    String(String),
    Bool(bool),
}

impl QueryValue {
    pub fn get_value_as_string(&self) -> Option<String> {
        match self {
            QueryValue::Number(s) => Some(s.to_string()),
            QueryValue::String(val) => Some(val.to_string()),
            QueryValue::Bool(val) => Some(val.to_string()),
        }
    }
}

// TODO: Can we make this less ugly?
impl PartialEq for QueryValue {
    fn eq(&self, other: &Self) -> bool {
        match self {
            QueryValue::Number(s) => {
                if let QueryValue::Number(s2) = other {
                    s == s2
                } else {
                    false
                }
            }
            QueryValue::String(a) => match other {
                QueryValue::String(b) => a == b,
                _ => false,
            },
            QueryValue::Bool(a) => match other {
                QueryValue::Bool(b) => a == b,
                _ => false,
            },
        }
    }
}

impl From<String> for QueryValue {
    fn from(value: String) -> Self {
        if let Ok(val) = value.parse::<f64>() {
            QueryValue::Number(val)
        } else if let Ok(value) = value.parse::<bool>() {
            QueryValue::Bool(value)
        } else {
            QueryValue::String(value)
        }
    }
}
