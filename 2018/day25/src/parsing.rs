use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending, space0},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::preceded,
};

pub type Item = (isize, isize, isize, isize);

fn parse_line(input: &str) -> IResult<&str, Item> {
    preceded(
        space0,
        separated_list1(tag(","), complete::isize).map(|v| (v[0], v[1], v[2], v[3])),
    )
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
