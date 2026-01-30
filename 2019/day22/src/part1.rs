use crate::parsing::{Item, parse_input};
use aoc_helper::nom::Sign;

pub fn process(_input: &str) -> usize {
    shuffle(10007, _input)
        .into_iter()
        .enumerate()
        .find_map(|(i, x)| (x == 2019).then_some(i))
        .unwrap()
}

fn shuffle(ncards: usize, input: &str) -> Vec<usize> {
    let input = parse_input(input);
    let mut stack: Vec<usize> = (0..ncards).collect();

    for item in input {
        match item {
            Item::Reverse => stack.reverse(),
            Item::Cut(s, u) => match s {
                Sign::Positive => {
                    let mut new_stack = stack[u..].to_vec();
                    new_stack.extend(&stack[..u]);
                    stack = new_stack;
                }
                Sign::Negative => {
                    let u = ncards - u;
                    let mut new_stack = stack[u..].to_vec();
                    new_stack.extend(&stack[..u]);
                    stack = new_stack;
                }
            },
            Item::Deal(x) => {
                let mut new_stack = vec![0; ncards];
                let mut pos = 0;
                for card in stack {
                    new_stack[pos % ncards] = card;
                    pos += x;
                }
                stack = new_stack;
            }
        }
    }

    stack
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        r#"deal with increment 7
deal into new stack
deal into new stack"#,
        "0 3 6 9 2 5 8 1 4 7"
    )]
    #[case(
        r#"cut 6
deal with increment 7
deal into new stack"#,
        "3 0 7 4 1 8 5 2 9 6"
    )]
    #[case(
        r#"deal with increment 7
deal with increment 9
cut -2"#,
        "6 3 0 7 4 1 8 5 2 9"
    )]
    #[case(
        r#"deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1"#,
        "9 2 5 8 1 4 7 0 3 6"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(
            shuffle(10, input),
            expected
                .split_ascii_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        );
    }
}
