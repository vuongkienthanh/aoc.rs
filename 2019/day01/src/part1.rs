use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    input.into_iter().map(|a| fuel(a)).sum()
}

fn fuel(a: usize) -> usize {
    a / 3 - 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(12, 2)]
    #[case(14, 2)]
    #[case(1969, 654)]
    #[case(100756, 33583)]
    fn test_fuel(#[case] input: usize, #[case] expected: usize) {
        assert_eq!(fuel(input), expected);
    }
}
