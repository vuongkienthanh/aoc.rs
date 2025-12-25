use crate::parsing::{Item, parse_input};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut sum = 0;
    count(input, 1, &mut sum);
    sum
}

fn count(item: Item, current_score: usize, sum: &mut usize) {
    if let Item::Group(v) = item {
        *sum += current_score;

        for i in v {
            count(i, current_score + 1, sum)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("{}", 1)]
    #[case("{{{}}}", 6)]
    #[case("{{},{}}", 5)]
    #[case("{{{},{},{{}}}}", 16)]
    #[case("{<a>,<a>,<a>,<a>}", 1)]
    #[case("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9)]
    #[case("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9)]
    #[case("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3)]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
