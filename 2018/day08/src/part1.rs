use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let mut input = parse_input(_input);
    input.reverse();
    sum_metadata(&mut input)
}

fn sum_metadata(input: &mut Vec<usize>) -> usize {
    let n_node = input.pop().unwrap();
    let n_metadata = input.pop().unwrap();

    let mut sum = 0;
    for _ in 0..n_node {
        sum += sum_metadata(input);
    }
    for _ in 0..n_metadata {
        sum += input.pop().unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 138);
    }
}
