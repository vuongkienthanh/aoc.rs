use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_while},
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

type Class<'a> = Vec<(&'a str, ((usize, usize), (usize, usize)))>;
fn parse_class(input: &str) -> IResult<&str, &str> {
    take_while(|x| x != ':').parse(input)
}
fn parse_range(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(complete::usize, tag("-"), complete::usize).parse(input)
}
fn parse_ranges(input: &str) -> IResult<&str, ((usize, usize), (usize, usize))> {
    separated_pair(parse_range, tag(" or "), parse_range).parse(input)
}
fn parse_class_line(input: &str) -> IResult<&str, (&str, ((usize, usize), (usize, usize)))> {
    separated_pair(parse_class, tag(": "), parse_ranges).parse(input)
}
fn parse_class_group<'a>(input: &'a str) -> IResult<&'a str, Class<'a>> {
    separated_list1(line_ending, parse_class_line).parse(input)
}
fn parse_ticket(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(","), complete::usize).parse(input)
}
fn parse_your_ticket(input: &str) -> IResult<&str, Vec<usize>> {
    preceded(tag("\n\nyour ticket:\n"), parse_ticket).parse(input)
}
fn parse_nearby_ticket(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    preceded(
        tag("\n\nnearby tickets:\n"),
        separated_list1(line_ending, parse_ticket),
    )
    .parse(input)
}

pub fn parse_input<'a>(input: &'a str) -> (Class<'a>, Vec<usize>, Vec<Vec<usize>>) {
    all_consuming((parse_class_group, parse_your_ticket, parse_nearby_ticket))
        .parse(input)
        .unwrap()
        .1
}
