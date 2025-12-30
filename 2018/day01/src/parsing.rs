use nom::{
    IResult, Parser,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};

type Item = isize;

fn parse_line(input: &str) -> IResult<&str, Item> {
    complete::isize.parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
