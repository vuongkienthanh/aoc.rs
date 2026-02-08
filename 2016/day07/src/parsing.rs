use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::delimited,
};

#[derive(Debug)]
pub enum Bracket<'a> {
    In(&'a str),
    Out(&'a str),
}
pub fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Vec<Bracket<'a>>> {
    many1(alt((
        alpha1.map(Bracket::Out),
        delimited(tag("["), alpha1, tag("]")).map(Bracket::In),
    )))
    .parse(input)
}

pub fn parse_input<'a>(input: &'a str) -> Vec<Vec<Bracket<'a>>> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
