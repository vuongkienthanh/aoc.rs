pub mod parsing;
pub mod part1;
pub mod part2;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Value {
    List(Vec<Value>),
    Int(u8),
}

impl Ord for Value {
    fn cmp(&self, other: &Value) -> Ordering {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.cmp(b),
            (Value::List(a), Value::List(b)) => a.cmp(b),
            (_, Value::Int(b)) => self.cmp(&Value::List(vec![Value::Int(*b)])),
            (Value::Int(a), _) => Value::List(vec![Value::Int(*a)]).cmp(other),
        }
    }
}
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parsing::parse_value;
    use rstest::*;

    #[rstest]
    #[case(Value::List(vec![Value::Int(1)]), Value::Int(1), Ordering::Equal)]
    #[case(parse_value("[[6]]").unwrap().1, parse_value("[[2]]").unwrap().1, Ordering::Greater)]
    fn test_process(#[case] a: Value, #[case] b: Value, #[case] expected: Ordering) {
        assert_eq!(a.cmp(&b), expected);
    }
}
