use nom::{
    IResult, Parser,
    character::complete::{self, space0, space1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::delimited,
};

fn parse_items(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(space0, separated_list1(space1, complete::usize), space0).parse(input)
}

pub fn parse_input(input: &str) -> Vec<usize> {
    all_consuming(parse_items).parse(input).unwrap().1
}
