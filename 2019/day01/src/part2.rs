use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    input.into_iter().map(|a| fuel(a)).sum()
}

fn fuel(a: usize) -> usize {
    let required = (a / 3).saturating_sub(2);
    if required == 0 {
        0
    } else {
        required + fuel(required)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(14, 2)]
    #[case(1969, 966)]
    #[case(100756, 50346)]
    fn test_fuel(#[case] input: usize, #[case] expected: usize) {
        assert_eq!(fuel(input), expected);
    }
}
