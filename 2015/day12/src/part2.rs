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
        Value::Object(m) => {
            if m.values().any(|v| v == "red") {
                0
            } else {
                m.into_values().map(count_number).sum()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("[1,2,3]", 6)]
    #[case(r#"[1,{"c":"red","b":2},3]"#, 4)]
    #[case(r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0)]
    #[case(r#"[1,"red",5]"#, 6)]
    fn test_process(#[case] input: &str, #[case] expected: isize) {
        assert_eq!(process(input), expected);
    }
}
