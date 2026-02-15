use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
};

fn parse_item(input: &str) -> IResult<&str, u8> {
    alpha1
        .map(|x: &str| {
            let mut a = 0;
            for c in x.bytes() {
                a |= 1 << (c - b'a');
            }
            a
        })
        .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, (Vec<u8>, Vec<u8>)> {
    separated_pair(
        separated_list1(tag(" "), parse_item),
        tag(" | "),
        separated_list1(tag(" "), parse_item),
    )
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<(Vec<u8>, Vec<u8>)> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
