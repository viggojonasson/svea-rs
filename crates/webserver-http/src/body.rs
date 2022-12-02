/// An http body.
#[derive(Debug, Clone, PartialEq)]
pub enum BodyValue {
    /// A body with a string.
    String(String),
    Empty,
}

pub fn parse_body(
    body: String,
    content_type: Option<&String>,
) -> Result<BodyValue, Box<dyn std::error::Error>> {
    if let Some(content_type) = content_type {}

    Ok(BodyValue::String(body))
}

impl ToString for BodyValue {
    fn to_string(&self) -> String {
        match self {
            BodyValue::String(s) => s.to_string(),
            BodyValue::Empty => String::new(),
        }
    }
}
