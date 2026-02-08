use nom::{
    IResult, Parser, bytes::complete::tag, character::complete, combinator::all_consuming,
    sequence::separated_pair,
};

fn parse_line(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(complete::usize, tag("-"), complete::usize).parse(input)
}

pub fn parse_input(input: &str) -> (usize, usize) {
    all_consuming(parse_line).parse(input).unwrap().1
}
