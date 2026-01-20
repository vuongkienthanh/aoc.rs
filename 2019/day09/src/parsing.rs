use nom::{
    IResult, Parser, bytes::complete::tag, character::complete, combinator::all_consuming,
    multi::separated_list1,
};

fn parse_item(input: &str) -> IResult<&str, i128> {
    complete::i128(input)
}

pub fn parse_input(input: &str) -> Vec<i128> {
    all_consuming(separated_list1(tag(","), parse_item))
        .parse(input)
        .unwrap()
        .1
}
