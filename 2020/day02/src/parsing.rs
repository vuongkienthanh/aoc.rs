use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};

fn parse_line(input: &str) -> IResult<&str, (usize, usize, char, &str)> {
    (
        complete::usize,
        tag("-"),
        complete::usize,
        tag(" "),
        complete::anychar,
        tag(": "),
        alpha1,
    )
        .map(|(a, _, b, _, c, _, d)| (a, b, c, d))
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<(usize, usize, char, &str)> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
