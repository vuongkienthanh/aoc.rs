use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::separated_pair,
};

fn parse_algo(input: &str) -> IResult<&str, Vec<char>> {
    many1(alt((complete::char('#'), complete::char('.')))).parse(input)
}

fn parse_image(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(
        line_ending,
        many1(alt((complete::char('#'), complete::char('.')))),
    )
    .parse(input)
}

pub fn parse_input(input: &str) -> (Vec<char>, Vec<Vec<char>>) {
    all_consuming(separated_pair(
        parse_algo,
        (line_ending, line_ending),
        parse_image,
    ))
    .parse(input)
    .unwrap()
    .1
}
