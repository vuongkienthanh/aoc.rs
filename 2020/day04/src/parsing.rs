use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
};

fn parse_item(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha1, tag(":"), take_while(|x| x != '\n' && x != ' ')).parse(input)
}
fn parse_passport(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(alt((line_ending, tag(" "))), parse_item)
        .map(|mut group| {
            group.sort_unstable_by_key(|(k, _)| *k);
            group
        })
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Vec<(&str, &str)>> {
    all_consuming(separated_list1((line_ending, line_ending), parse_passport))
        .parse(input)
        .unwrap()
        .1
}
