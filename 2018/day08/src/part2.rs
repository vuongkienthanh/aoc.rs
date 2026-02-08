use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let mut input = parse_input(_input);
    input.reverse();
    node_value(&mut input)
}
fn node_value(input: &mut Vec<usize>) -> usize {
    let n_node = input.pop().unwrap();
    let n_metadata = input.pop().unwrap();

    if n_node == 0 {
        (0..n_metadata).map(|_| input.pop().unwrap()).sum()
    } else {
        let children: Vec<usize> = (0..n_node).map(|_| node_value(input)).collect();

        (0..n_metadata)
            .map(|_| {
                let i = input.pop().unwrap() - 1;
                children.get(i).cloned().unwrap_or_default()
            })
            .sum()
    }
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
        assert_eq!(process(fixture), 66);
    }
}
