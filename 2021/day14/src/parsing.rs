use fxhash::FxHashMap;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

pub type Map = FxHashMap<(char, char), char>;

fn parse_template(input: &str) -> IResult<&str, Vec<char>> {
    alpha1.map(|x: &str| x.chars().collect()).parse(input)
}
fn parse_line(input: &str) -> IResult<&str, ((char, char), char)> {
    ((anychar, anychar), preceded(tag(" -> "), anychar)).parse(input)
}
fn parse_lines(input: &str) -> IResult<&str, Map> {
    separated_list1(line_ending, parse_line)
        .map(|v| v.into_iter().collect())
        .parse(input)
}

pub fn parse_input(input: &str) -> (Vec<char>, Map) {
    all_consuming(separated_pair(
        parse_template,
        (line_ending, line_ending),
        parse_lines,
    ))
    .parse(input)
    .unwrap()
    .1
}
