use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::preceded,
};

pub type Item<'a> = (&'a str, Vec<&'a str>);

fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Item<'a>> {
    (alpha1, tag(":"), many1(preceded(space1, alpha1)))
        .map(|(a, _, b)| (a, b))
        .parse(input)
}

pub fn parse_input<'a>(input: &'a str) -> Vec<Item<'a>> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
