use crate::parsing::parse_input;
use serde_json::Value;

pub fn process(_input: &str) -> isize {
    let input = parse_input(_input);
    count_number(input)
}

fn count_number(input: Value) -> isize {
    match input {
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
        Value::Number(n) => n.as_i64().expect("should be a valid json number") as isize,
        Value::Array(v) => v.into_iter().map(count_number).sum(),
        Value::Object(m) => m.into_values().map(count_number).sum(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("[1,2,3]", 6)]
    #[case(r#"{"a":2,"b":4}"#, 6)]
    #[case("[[[3]]]", 3)]
    #[case(r#"{"a":{"b":4},"c":-1}"#, 3)]
    #[case(r#"{"a":[-1,1]}"#, 0)]
    #[case(r#"[-1,{"a":1}]"#, 0)]
    #[case("[]", 0)]
    #[case("{}", 0)]
    fn test_process(#[case] input: &str, #[case] expected: isize) {
        assert_eq!(process(input), expected);
    }
}
