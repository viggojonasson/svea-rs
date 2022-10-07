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

impl TryFrom<String> for Query {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Ok(value) = value.parse::<u8>() {
            Ok(Query::U8(value))
        } else if let Ok(value) = value.parse::<u16>() {
            Ok(Query::U16(value))
        } else if let Ok(value) = value.parse::<u32>() {
            Ok(Query::U32(value))
        } else if let Ok(value) = value.parse::<u64>() {
            Ok(Query::U64(value))
        } else if let Ok(value) = value.parse::<i8>() {
            Ok(Query::I8(value))
        } else if let Ok(value) = value.parse::<i16>() {
            Ok(Query::I16(value))
        } else if let Ok(value) = value.parse::<i32>() {
            Ok(Query::I32(value))
        } else if let Ok(value) = value.parse::<i64>() {
            Ok(Query::I64(value))
        } else if let Ok(value) = value.parse::<f32>() {
            Ok(Query::F32(value))
        } else if let Ok(value) = value.parse::<f64>() {
            Ok(Query::F64(value))
        } else if let Ok(value) = value.parse::<bool>() {
            Ok(Query::Bool(value))
        } else {
            Ok(Query::String(value))
        }
    }
}
