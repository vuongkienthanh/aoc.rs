use crate::{CD, CMD, LS};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{self, alpha1, line_ending},
    combinator::all_consuming,
    multi::{separated_list0, separated_list1},
    sequence::{preceded, terminated},
};

fn parse_cd_type<'a>(input: &'a str) -> IResult<&'a str, CD<'a>> {
    alt((tag("..").map(|_| CD::Back), alpha1.map(CD::Path))).parse(input)
}

fn parse_cd<'a>(input: &'a str) -> IResult<&'a str, CMD<'a>> {
    preceded(tag("$ cd "), parse_cd_type)
        .map(CMD::cd)
        .parse(input)
}

fn parse_ls_type<'a>(input: &'a str) -> IResult<&'a str, LS<'a>> {
    alt((
        preceded(tag("dir "), alpha1).map(LS::Dir),
        terminated(complete::usize, take_while(|x| x != '\n')).map(LS::File),
    ))
    .parse(input)
}
fn parse_ls<'a>(input: &'a str) -> IResult<&'a str, CMD<'a>> {
    preceded(tag("$ ls\n"), separated_list0(line_ending, parse_ls_type))
        .map(CMD::ls)
        .parse(input)
}

fn parse_cmd<'a>(input: &'a str) -> IResult<&'a str, CMD<'a>> {
    alt((parse_cd, parse_ls)).parse(input)
}

pub fn parse_input<'a>(input: &'a str) -> Vec<CMD<'a>> {
    all_consuming(preceded(
        tag("$ cd /\n"),
        separated_list1(line_ending, parse_cmd),
    ))
    .parse(input)
    .unwrap()
    .1
}
