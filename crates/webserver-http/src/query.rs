#[derive(Clone, Debug)]
pub enum QueryValue {
    U8(Option<u8>),
    U16(Option<u16>),
    U32(Option<u32>),
    U64(Option<u64>),
    I8(Option<i8>),
    I16(Option<i16>),
    I32(Option<i32>),
    I64(Option<i64>),
    F32(Option<f32>),
    F64(Option<f64>),
    String(Option<String>),
    Bool(Option<bool>),
}

/// TODO: Can we make this less ugly?
impl PartialEq for QueryValue {
    fn eq(&self, other: &Self) -> bool {
        match self {
            QueryValue::U8(a) => match other {
                QueryValue::U8(b) => match a {
                    Some(a) => match b {
                        Some(b) => a == b,
                        None => true,
                    },
                    None => true,
                },
                _ => false,
            },
            QueryValue::U16(a) => match other {
                QueryValue::U16(b) => match a {
                    Some(a) => match b {
                        Some(b) => a == b,
                        None => true,
                    },
                    None => true,
                },
                _ => false,
            },
            QueryValue::U32(a) => match other {
                QueryValue::U32(b) => match a {
                    Some(a) => match b {
                        Some(b) => a == b,
                        None => true,
                    },
                    None => true,
                },
                _ => false,
            },
            QueryValue::U64(a) => match other {
                QueryValue::U64(b) => match a {
                    Some(a) => match b {
                        Some(b) => a == b,
                        None => true,
                    },
                    None => true,
                },
                _ => false,
            },
            QueryValue::I8(a) => match other {
                QueryValue::I8(b) => match a {
                    Some(a) => match b {
                        Some(b) => a == b,
                        None => true,
                    },
                    None => true,
                },
                _ => false,
            },
            QueryValue::I16(a) => match other {
                QueryValue::I16(b) => match a {
                    Some(a) => match b {
                        Some(b) => a == b,
                        None => true,
                    },
                    None => true,
                },
                _ => false,
            },
            QueryValue::I32(a) => match other {
                QueryValue::I32(b) => match a {
                    Some(a) => match b {
                        Some(b) => a == b,
                        None => true,
                    },
                    None => true,
                },
                _ => false,
            },
            QueryValue::I64(a) => match other {
                QueryValue::I64(b) => match a {
                    Some(a) => match b {
                        Some(b) => a == b,
                        None => true,
                    },
                    None => true,
                },
                _ => false,
            },
            QueryValue::F32(a) => match other {
                QueryValue::F32(b) => match a {
                    Some(a) => match b {
                        Some(b) => a == b,
                        None => true,
                    },
                    None => true,
                },
                _ => false,
            },
            QueryValue::F64(a) => match other {
                QueryValue::F64(b) => match a {
                    Some(a) => match b {
                        Some(b) => a == b,
                        None => true,
                    },
                    None => true,
                },
                _ => false,
            },
            QueryValue::String(a) => match other {
                QueryValue::String(b) => match a {
                    Some(a) => match b {
                        Some(b) => a == b,
                        None => true,
                    },
                    None => true,
                },
                _ => false,
            },
            QueryValue::Bool(a) => match other {
                QueryValue::Bool(b) => match a {
                    Some(a) => match b {
                        Some(b) => a == b,
                        None => true,
                    },
                    None => true,
                },
                _ => false,
            },
        }
    }
}

impl From<String> for QueryValue {
    fn from(value: String) -> Self {
        if let Ok(value) = value.parse::<u8>() {
            QueryValue::U8(Some(value))
        } else if let Ok(value) = value.parse::<u16>() {
            QueryValue::U16(Some(value))
        } else if let Ok(value) = value.parse::<u32>() {
            QueryValue::U32(Some(value))
        } else if let Ok(value) = value.parse::<u64>() {
            QueryValue::U64(Some(value))
        } else if let Ok(value) = value.parse::<i8>() {
            QueryValue::I8(Some(value))
        } else if let Ok(value) = value.parse::<i16>() {
            QueryValue::I16(Some(value))
        } else if let Ok(value) = value.parse::<i32>() {
            QueryValue::I32(Some(value))
        } else if let Ok(value) = value.parse::<i64>() {
            QueryValue::I64(Some(value))
        } else if let Ok(value) = value.parse::<f32>() {
            QueryValue::F32(Some(value))
        } else if let Ok(value) = value.parse::<f64>() {
            QueryValue::F64(Some(value))
        } else if let Ok(value) = value.parse::<bool>() {
            QueryValue::Bool(Some(value))
        } else {
            QueryValue::String(Some(value))
        }
    }
}
