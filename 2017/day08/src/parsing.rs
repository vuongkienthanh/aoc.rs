use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
};
// https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md

#[derive(Debug)]
pub enum Cond<'a> {
    GT(&'a str, isize),
    GE(&'a str, isize),
    LT(&'a str, isize),
    LE(&'a str, isize),
    EQ(&'a str, isize),
    NEQ(&'a str, isize),
}
#[derive(Debug)]
pub enum Op<'a> {
    Inc(&'a str, isize),
    Dec(&'a str, isize),
}

pub type Item<'a> = (Op<'a>, Cond<'a>);

fn parse_inc<'a>(input: &'a str) -> IResult<&'a str, Op<'a>> {
    separated_pair(alpha1, tag(" inc "), complete::isize)
        .map(|(a, b)| Op::Inc(a, b))
        .parse(input)
}
fn parse_dec<'a>(input: &'a str) -> IResult<&'a str, Op<'a>> {
    separated_pair(alpha1, tag(" dec "), complete::isize)
        .map(|(a, b)| Op::Dec(a, b))
        .parse(input)
}
fn parse_gt<'a>(input: &'a str) -> IResult<&'a str, Cond<'a>> {
    (tag(" if "), alpha1, tag(" > "), complete::isize)
        .map(|(_, a, _, b)| Cond::GT(a, b))
        .parse(input)
}
fn parse_ge<'a>(input: &'a str) -> IResult<&'a str, Cond<'a>> {
    (tag(" if "), alpha1, tag(" >= "), complete::isize)
        .map(|(_, a, _, b)| Cond::GE(a, b))
        .parse(input)
}
fn parse_lt<'a>(input: &'a str) -> IResult<&'a str, Cond<'a>> {
    (tag(" if "), alpha1, tag(" < "), complete::isize)
        .map(|(_, a, _, b)| Cond::LT(a, b))
        .parse(input)
}
fn parse_le<'a>(input: &'a str) -> IResult<&'a str, Cond<'a>> {
    (tag(" if "), alpha1, tag(" <= "), complete::isize)
        .map(|(_, a, _, b)| Cond::LE(a, b))
        .parse(input)
}
fn parse_eq<'a>(input: &'a str) -> IResult<&'a str, Cond<'a>> {
    (tag(" if "), alpha1, tag(" == "), complete::isize)
        .map(|(_, a, _, b)| Cond::EQ(a, b))
        .parse(input)
}
fn parse_neq<'a>(input: &'a str) -> IResult<&'a str, Cond<'a>> {
    (tag(" if "), alpha1, tag(" != "), complete::isize)
        .map(|(_, a, _, b)| Cond::NEQ(a, b))
        .parse(input)
}

fn parse_op<'a>(input: &'a str) -> IResult<&'a str, Op<'a>> {
    alt((parse_inc, parse_dec)).parse(input)
}
fn parse_cond<'a>(input: &'a str) -> IResult<&'a str, Cond<'a>> {
    alt((parse_gt, parse_ge, parse_lt, parse_le, parse_eq, parse_neq)).parse(input)
}

fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Item<'a>> {
    (parse_op, parse_cond).parse(input)
}

pub fn parse_input<'a>(input: &'a str) -> Vec<Item<'a>> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
