use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

#[derive(Debug)]
pub enum Operand<'a> {
    Value(u16),
    Name(&'a str),
}

#[derive(Debug)]
pub enum Operation<'a> {
    Assign(Operand<'a>, &'a str),
    And(Operand<'a>, Operand<'a>, &'a str),
    Or(Operand<'a>, Operand<'a>, &'a str),
    Lshift(&'a str, u16, &'a str),
    Rshift(&'a str, u16, &'a str),
    Not(&'a str, &'a str),
}

fn parse_operand<'a>(input: &'a str) -> IResult<&'a str, Operand<'a>> {
    alt((
        digit1.map(|x: &str| Operand::Value(x.parse::<u16>().unwrap())),
        alpha1.map(|x: &str| Operand::Name(x)),
    ))
    .parse(input)
}

fn parse_assign<'a>(input: &'a str) -> IResult<&'a str, Operation<'a>> {
    separated_pair(parse_operand, tag(" -> "), alpha1)
        .map(|(x, y)| Operation::Assign(x, y))
        .parse(input)
}
fn parse_and<'a>(input: &'a str) -> IResult<&'a str, Operation<'a>> {
    separated_pair(
        separated_pair(parse_operand, tag(" AND "), parse_operand),
        tag(" -> "),
        alpha1,
    )
    .map(|((x, y), z)| Operation::And(x, y, z))
    .parse(input)
}
fn parse_or<'a>(input: &'a str) -> IResult<&'a str, Operation<'a>> {
    separated_pair(
        separated_pair(parse_operand, tag(" OR "), parse_operand),
        tag(" -> "),
        alpha1,
    )
    .map(|((x, y), z)| Operation::Or(x, y, z))
    .parse(input)
}
fn parse_lshift<'a>(input: &'a str) -> IResult<&'a str, Operation<'a>> {
    separated_pair(
        separated_pair(alpha1, tag(" LSHIFT "), map_res(digit1, str::parse)),
        tag(" -> "),
        alpha1,
    )
    .map(|((x, y), z)| Operation::Lshift(x, y, z))
    .parse(input)
}
fn parse_rshift<'a>(input: &'a str) -> IResult<&'a str, Operation<'a>> {
    separated_pair(
        separated_pair(alpha1, tag(" RSHIFT "), map_res(digit1, str::parse)),
        tag(" -> "),
        alpha1,
    )
    .map(|((x, y), z)| Operation::Rshift(x, y, z))
    .parse(input)
}
fn parse_not<'a>(input: &'a str) -> IResult<&'a str, Operation<'a>> {
    separated_pair(preceded(tag("NOT "), alpha1), tag(" -> "), alpha1)
        .map(|(x, y)| Operation::Not(x, y))
        .parse(input)
}

fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Operation<'a>> {
    alt((
        parse_assign,
        parse_and,
        parse_or,
        parse_lshift,
        parse_rshift,
        parse_not,
    ))
    .parse(input)
}
pub fn parse_input<'a>(input: &'a str) -> IResult<&'a str, Vec<Operation<'a>>> {
    separated_list1(line_ending, parse_line).parse(input)
}
