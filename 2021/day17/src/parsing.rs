use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete,
    combinator::all_consuming,
    sequence::preceded,
};

fn parse_line(input: &str) -> IResult<&str, (isize, isize, isize, isize)> {
    (
        preceded(tag("target area: x="), complete::isize),
        preceded(tag(".."), complete::isize),
        preceded(tag(", y="), complete::isize),
        preceded(tag(".."), complete::isize),
    )
        .parse(input)
}

pub fn parse_input(input: &str) -> (isize, isize, isize, isize) {
    all_consuming(parse_line).parse(input).unwrap().1
}
