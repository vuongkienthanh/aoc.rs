use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::delimited,
};

pub type Item<'a> = (Vec<&'a str>, Vec<&'a str>);

fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Item<'a>> {
    (
        separated_list1(tag(" "), alpha1),
        delimited(
            tag(" (contains "),
            separated_list1(tag(", "), alpha1),
            tag(")"),
        ),
    )
        .parse(input)
}

pub fn parse_input<'a>(input: &'a str) -> Vec<Item<'a>> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
