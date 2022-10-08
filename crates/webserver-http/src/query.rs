#[derive(Clone, Debug, PartialEq)]
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
