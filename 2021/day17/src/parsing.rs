use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete,
    combinator::all_consuming,
    sequence::preceded,
};

fn parse_line(input: &str) -> IResult<&str, (usize, usize, usize, usize)> {
    (
        preceded(tag("target area: x="), complete::usize),
        preceded(tag(".."), complete::usize),
        preceded(tag(", y=-"), complete::usize),
        preceded(tag("..-"), complete::usize),
    )
        .parse(input)
}

pub fn parse_input(input: &str) -> (usize, usize, usize, usize) {
    all_consuming(parse_line).parse(input).unwrap().1
}