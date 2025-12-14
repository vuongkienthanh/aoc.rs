use nom::{
    IResult, Parser,
    bytes::complete::take_while,
    character::complete::{self, char, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::delimited,
};

type Item = (usize, Vec<Vec<usize>>, Vec<usize>);

fn parse_bulbs(input: &str) -> IResult<&str, usize> {
    delimited(char('['), take_while(|x| x == '.' || x == '#'), char(']'))
        .map(|x: &str| {
            let mut ans = 0;
            for (i, c) in x.char_indices() {
                if c == '#' {
                    ans |= 1 << i
                }
            }
            ans
        })
        .parse(input)
}
fn parse_button(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        char('('),
        separated_list1(char(','), complete::usize),
        char(')'),
    )
    .parse(input)
}
fn parse_buttons(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(char(' '), parse_button).parse(input)
}
fn parse_joltage(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        char('{'),
        separated_list1(char(','), complete::usize),
        char('}'),
    )
    .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Item> {
    (
        parse_bulbs,
        char(' '),
        parse_buttons,
        char(' '),
        parse_joltage,
    )
        .map(|(a, _, b, _, c)| (a, b, c))
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
