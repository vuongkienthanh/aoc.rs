use serde_json;

pub fn parse_input(input: &str) -> serde_json::Value {
    serde_json::from_str(input).expect("a valid JSON object")
}
