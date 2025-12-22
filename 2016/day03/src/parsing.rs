use nom::{
    IResult, Parser,
    character::complete::{self, line_ending, space0},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::preceded,
};

type Item = (usize, usize, usize);

fn parse_line(input: &str) -> IResult<&str, Item> {
    (
        preceded(space0, complete::usize),
        preceded(space0, complete::usize),
        preceded(space0, complete::usize),
    )
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
