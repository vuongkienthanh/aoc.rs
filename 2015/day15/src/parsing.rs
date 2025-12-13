use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
};

fn parse_number(input: &str) -> IResult<&str, isize> {
    alt((
        (tag("-"), digit1).map(|(_, i): (_, &str)| -i.parse::<isize>().unwrap()),
        map_res(digit1, str::parse),
    ))
    .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, [isize; 5]> {
    (
        alpha1,
        tag(": capacity "),
        parse_number,
        tag(", durability "),
        parse_number,
        tag(", flavor "),
        parse_number,
        tag(", texture "),
        parse_number,
        tag(", calories "),
        parse_number,
    )
        .map(|(_, _, a, _, b, _, c, _, d, _, e)| [a, b, c, d, e])
        .parse(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<[isize; 5]>> {
    separated_list1(line_ending, parse_line).parse(input)
}
