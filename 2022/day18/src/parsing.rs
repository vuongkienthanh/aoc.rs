use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};

pub type Item = (i8, i8, i8);

fn parse_line(input: &str) -> IResult<&str, Item> {
    (complete::i8, tag(","), complete::i8, tag(","), complete::i8)
        .map(|(a, _, b, _, c)| (a, b, c))
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
