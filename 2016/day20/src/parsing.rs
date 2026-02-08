use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
};

type Item = (u32, u32);

fn parse_line(input: &str) -> IResult<&str, Item> {
    let (input, (a, b)) = separated_pair(complete::u32, tag("-"), complete::u32).parse(input)?;
    assert!(a < b);
    Ok((input, (a, b)))
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
