#[derive(Clone, Debug, PartialEq)]
pub enum Query {
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

impl From<String> for Query {
    fn from(value: String) -> Self {
        if let Ok(value) = value.parse::<u8>() {
            Query::U8(value)
        } else if let Ok(value) = value.parse::<u16>() {
            Query::U16(value)
        } else if let Ok(value) = value.parse::<u32>() {
            Query::U32(value)
        } else if let Ok(value) = value.parse::<u64>() {
            Query::U64(value)
        } else if let Ok(value) = value.parse::<i8>() {
            Query::I8(value)
        } else if let Ok(value) = value.parse::<i16>() {
            Query::I16(value)
        } else if let Ok(value) = value.parse::<i32>() {
            Query::I32(value)
        } else if let Ok(value) = value.parse::<i64>() {
            Query::I64(value)
        } else if let Ok(value) = value.parse::<f32>() {
            Query::F32(value)
        } else if let Ok(value) = value.parse::<f64>() {
            Query::F64(value)
        } else if let Ok(value) = value.parse::<bool>() {
            Query::Bool(value)
        } else {
            Query::String(value)
        }
    }
}
