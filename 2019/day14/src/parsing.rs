use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
};

pub type Item<'a> = (usize, &'a str);

fn parse_item<'a>(input: &'a str) -> IResult<&'a str, Item<'a>> {
    separated_pair(complete::usize, tag(" "), alpha1).parse(input)
}
fn parse_items<'a>(input: &'a str) -> IResult<&'a str, Vec<Item<'a>>> {
    separated_list1(tag(", "), parse_item).parse(input)
}

fn parse_line<'a>(input: &'a str) -> IResult<&'a str, (Vec<Item<'a>>, Item<'a>)> {
    separated_pair(parse_items, tag(" => "), parse_item).parse(input)
}

pub fn parse_input<'a>(input: &'a str) -> Vec<(Vec<Item<'a>>, Item<'a>)> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
