use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
};

fn parse_line(input: &str) -> IResult<&str, (char, char)> {
    separated_pair(complete::anychar, tag(" "), complete::anychar).parse(input)
}

pub fn parse_input(input: &str) -> Vec<(char, char)> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
