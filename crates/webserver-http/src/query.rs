#[derive(Clone, Debug)]
pub enum QueryValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
    Bool(bool),
}

impl QueryValue {
    pub fn get_value_as_string(&self) -> Option<String> {
        match self {
            QueryValue::U8(val) => Some(val.to_string()),
            QueryValue::U16(val) => Some(val.to_string()),
            QueryValue::U32(val) => Some(val.to_string()),
            QueryValue::U64(val) => Some(val.to_string()),
            QueryValue::I8(val) => Some(val.to_string()),
            QueryValue::I16(val) => Some(val.to_string()),
            QueryValue::I32(val) => Some(val.to_string()),
            QueryValue::I64(val) => Some(val.to_string()),
            QueryValue::F32(val) => Some(val.to_string()),
            QueryValue::F64(val) => Some(val.to_string()),
            QueryValue::String(val) => Some(val.to_string()),
            QueryValue::Bool(val) => Some(val.to_string()),
        }
    }
}

// TODO: Can we make this less ugly?
impl PartialEq for QueryValue {
    fn eq(&self, other: &Self) -> bool {
        match self {
            QueryValue::U8(a) => match other {
                QueryValue::U8(b) => a == b,
                _ => false,
            },
            QueryValue::U16(a) => match other {
                QueryValue::U16(b) => a == b,
                _ => false,
            },
            QueryValue::U32(a) => match other {
                QueryValue::U32(b) => a == b,
                _ => false,
            },
            QueryValue::U64(a) => match other {
                QueryValue::U64(b) => a == b,
                _ => false,
            },
            QueryValue::I8(a) => match other {
                QueryValue::I8(b) => a == b,
                _ => false,
            },
            QueryValue::I16(a) => match other {
                QueryValue::I16(b) => a == b,
                _ => false,
            },
            QueryValue::I32(a) => match other {
                QueryValue::I32(b) => a == b,
                _ => false,
            },
            QueryValue::I64(a) => match other {
                QueryValue::I64(b) => a == b,
                _ => false,
            },
            QueryValue::F32(a) => match other {
                QueryValue::F32(b) => a == b,
                _ => false,
            },
            QueryValue::F64(a) => match other {
                QueryValue::F64(b) => a == b,
                _ => false,
            },
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
        if let Ok(value) = value.parse::<u8>() {
            QueryValue::U8(value)
        } else if let Ok(value) = value.parse::<u16>() {
            QueryValue::U16(value)
        } else if let Ok(value) = value.parse::<u32>() {
            QueryValue::U32(value)
        } else if let Ok(value) = value.parse::<u64>() {
            QueryValue::U64(value)
        } else if let Ok(value) = value.parse::<i8>() {
            QueryValue::I8(value)
        } else if let Ok(value) = value.parse::<i16>() {
            QueryValue::I16(value)
        } else if let Ok(value) = value.parse::<i32>() {
            QueryValue::I32(value)
        } else if let Ok(value) = value.parse::<i64>() {
            QueryValue::I64(value)
        } else if let Ok(value) = value.parse::<f32>() {
            QueryValue::F32(value)
        } else if let Ok(value) = value.parse::<f64>() {
            QueryValue::F64(value)
        } else if let Ok(value) = value.parse::<bool>() {
            QueryValue::Bool(value)
        } else {
            QueryValue::String(value)
        }
    }
}
