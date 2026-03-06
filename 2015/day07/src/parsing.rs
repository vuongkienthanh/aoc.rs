use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

#[derive(Debug, Clone)]
pub enum Operand<'a> {
    Value(u16),
    Name(&'a str),
}

#[derive(Debug, Clone)]
pub enum OperandList<'a> {
    One(Operand<'a>),
    Two(Operand<'a>, Operand<'a>),
}

#[derive(Debug, Clone)]
pub enum Op {
    Assign,
    And,
    Or,
    Lshift,
    Rshift,
    Not,
}

pub type Item<'a> = (Op, OperandList<'a>, &'a str);

fn parse_operand<'a>(input: &'a str) -> IResult<&'a str, Operand<'a>> {
    alt((complete::u16.map(Operand::Value), alpha1.map(Operand::Name))).parse(input)
}

fn parse_assign<'a>(input: &'a str) -> IResult<&'a str, Item<'a>> {
    separated_pair(parse_operand, tag(" -> "), alpha1)
        .map(|(x, y)| (Op::Assign, OperandList::One(x), y))
        .parse(input)
}

fn parse_and<'a>(input: &'a str) -> IResult<&'a str, Item<'a>> {
    separated_pair(
        separated_pair(parse_operand, tag(" AND "), parse_operand),
        tag(" -> "),
        alpha1,
    )
    .map(|((x, y), z)| (Op::And, OperandList::Two(x, y), z))
    .parse(input)
}

fn parse_or<'a>(input: &'a str) -> IResult<&'a str, Item<'a>> {
    separated_pair(
        separated_pair(parse_operand, tag(" OR "), parse_operand),
        tag(" -> "),
        alpha1,
    )
    .map(|((x, y), z)| (Op::Or, OperandList::Two(x, y), z))
    .parse(input)
}

fn parse_lshift<'a>(input: &'a str) -> IResult<&'a str, Item<'a>> {
    separated_pair(
        separated_pair(parse_operand, tag(" LSHIFT "), parse_operand),
        tag(" -> "),
        alpha1,
    )
    .map(|((x, y), z)| (Op::Lshift, OperandList::Two(x, y), z))
    .parse(input)
}

fn parse_rshift<'a>(input: &'a str) -> IResult<&'a str, Item<'a>> {
    separated_pair(
        separated_pair(parse_operand, tag(" RSHIFT "), parse_operand),
        tag(" -> "),
        alpha1,
    )
    .map(|((x, y), z)| (Op::Rshift, OperandList::Two(x, y), z))
    .parse(input)
}

fn parse_not<'a>(input: &'a str) -> IResult<&'a str, Item<'a>> {
    separated_pair(preceded(tag("NOT "), parse_operand), tag(" -> "), alpha1)
        .map(|(x, y)| (Op::Not, OperandList::One(x), y))
        .parse(input)
}

fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Item<'a>> {
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

pub fn parse_input<'a>(input: &'a str) -> Vec<Item<'a>> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
