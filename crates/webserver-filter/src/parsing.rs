use crate::{QueryType, RequestBody};

pub fn parse_query(input: String) -> Vec<(String, QueryType)> {
    let mut input = input;
    let mut result: Vec<(String, QueryType)> = Vec::new();

    input = input.replace("?", "");

    for query in input.split('&') {
        let mut query = query.split('=');
        let key = query.next().unwrap().to_string();
        let value = query.next().unwrap().to_string();

        let value = if value.contains(',') {
            let mut array: Vec<QueryType> = Vec::new();
            for value in value.split(',') {
                array.push(parse_value(value.to_string()));
            }
            QueryType::Array(array)
        } else {
            parse_value(value)
        };

        result.push((key, value));
    }

    result
}

fn parse_value(value: String) -> QueryType {
    if value == "true" {
        QueryType::Boolean(true)
    } else if value == "false" {
        QueryType::Boolean(false)
    } else if value.parse::<f64>().is_ok() {
        QueryType::Number(value.parse::<f64>().unwrap())
    } else {
        QueryType::String(value)
    }
}

#[cfg(test)]
mod test {
    use super::{parse_query, QueryType};

    fn create_string() -> String {
        "?string=hello&number=20.30&boolean=true&array=1,2,3,4,5".to_string()
    }

    #[test]
    fn array() {
        let r = parse_query(create_string());
        let r = r.get(3).unwrap();
        assert_eq!(r.0, "array");
        assert_eq!(
            r.1,
            QueryType::Array(vec![
                QueryType::Number(1.0),
                QueryType::Number(2.0),
                QueryType::Number(3.0),
                QueryType::Number(4.0),
                QueryType::Number(5.0)
            ])
        );
    }

    #[test]
    fn strings() {
        let r = parse_query(create_string());
        let r = r.get(0).unwrap();
        assert_eq!(r.0, "string".to_string());
        assert_eq!(r.1, QueryType::String("hello".to_string()));
    }

    #[test]
    fn numbers() {
        let r = parse_query(create_string());
        let r = r.get(1).unwrap();
        assert_eq!(r.0, "number".to_string());
        assert_eq!(r.1, QueryType::Number(20.30));
    }

    #[test]
    fn booleans() {
        let r = parse_query(create_string());
        let r = r.get(2).unwrap();
        assert_eq!(r.0, "boolean".to_string());
        assert_eq!(r.1, QueryType::Boolean(true));
    }
}
