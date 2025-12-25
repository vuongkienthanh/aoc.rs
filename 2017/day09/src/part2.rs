use crate::parsing::{Item, parse_input};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    let mut sum = 0;
    count(input, &mut sum);
    sum
}

fn count(item: Item, sum: &mut usize) {
    match item {
        Item::Garbage(x) => *sum += x,
        Item::Group(v) => {
            for i in v {
                count(i, sum)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("<>", 0)]
    #[case("<random characters>", 17)]
    #[case("<<<<>", 3)]
    #[case("<{!>}>", 2)]
    #[case("<!!>", 0)]
    #[case("<!!!>>", 0)]
    #[case(r#"<{o"i!a,<{i<a>"#, 10)]
    fn test_process(#[case] input: &str, #[case] expect: usize) {
        assert_eq!(process(input), expect)
    }
}
