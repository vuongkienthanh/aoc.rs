use crate::parsing::parse_input;
use aoc_helper::range::merge;

pub fn process(_input: &str) -> u32 {
    let (_rest, input) = parse_input(_input).unwrap();
    // println!("{input:?}");
    assert!(_rest.is_empty());

    println!("before merge len={}", input.len());
    let mut input = merge(input);
    println!("after merge len={}", input.len());
    input.sort_unstable_by_key(|x| x.0);
    println!("after merge {input:?}");

    if input[0].0 > 0 { 0 } else { input[0].1 + 1 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"5-8
0-2
4-7"#
    }
    #[rstest]
    fn test_process(fixture: &str) {
        assert_eq!(process(fixture), 3);
    }
}
