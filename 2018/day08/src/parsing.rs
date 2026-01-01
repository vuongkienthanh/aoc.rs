use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self},
    combinator::all_consuming,
    multi::separated_list1,
};

fn parse_item(input: &str) -> IResult<&str, usize> {
    complete::usize.parse(input)
}

pub fn parse_input(input: &str) -> Vec<usize> {
    all_consuming(separated_list1(tag(" "), parse_item))
        .parse(input)
        .unwrap()
        .1
}
