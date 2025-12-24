use nom::{
    IResult, Parser,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};

fn parse_line(input: &str) -> IResult<&str, isize> {
    complete::isize(input)
}

pub fn parse_input(input: &str) -> Vec<isize> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
