use crate::{process_input, process_output, round64};

pub fn process(_input: &str) -> String {
    let input = process_input(_input);
    let output = round64(input);
    process_output(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("", "a2582a3a0e66e6e86e3812dcb672a272".to_string())]
    #[case("AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd".to_string())]
    #[case("1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d".to_string())]
    #[case("1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e".to_string())]
    fn test_process(#[case] input: &str, #[case] expect: String) {
        assert_eq!(process(input), expect);
    }
}
