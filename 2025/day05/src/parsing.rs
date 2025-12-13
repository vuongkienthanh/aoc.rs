use nom::{
    IResult, Parser,
    character::complete::{self, char, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{pair, separated_pair},
};

fn parse_range(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(complete::usize, char('-'), complete::usize).parse(input)
}

pub fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    all_consuming(separated_pair(
        separated_list1(line_ending, parse_range),
        pair(line_ending, line_ending),
        separated_list1(line_ending, complete::usize),
    ))
    .parse(input)
    .unwrap()
    .1
}
