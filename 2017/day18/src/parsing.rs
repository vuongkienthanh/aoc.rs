use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::preceded,
};

#[derive(Debug)]
pub enum Target {
    Register(char),
    Value(isize),
}

#[derive(Debug)]
pub enum Item {
    Snd(Target),
    Set(char, Target),
    Add(char, Target),
    Mul(char, Target),
    Mod(char, Target),
    Rcv(char),
    Jgz(Target, Target),
}

fn parse_target(input: &str) -> IResult<&str, Target> {
    alt((
        complete::isize.map(Target::Value),
        complete::anychar.map(Target::Register),
    ))
    .parse(input)
}

fn parse_snd(input: &str) -> IResult<&str, Item> {
    preceded(tag("snd "), parse_target)
        .map(Item::Snd)
        .parse(input)
}
fn parse_set(input: &str) -> IResult<&str, Item> {
    (tag("set "), complete::anychar, tag(" "), parse_target)
        .map(|(_, a, _, b)| Item::Set(a, b))
        .parse(input)
}
fn parse_add(input: &str) -> IResult<&str, Item> {
    (tag("add "), complete::anychar, tag(" "), parse_target)
        .map(|(_, a, _, b)| Item::Add(a, b))
        .parse(input)
}
fn parse_mul(input: &str) -> IResult<&str, Item> {
    (tag("mul "), complete::anychar, tag(" "), parse_target)
        .map(|(_, a, _, b)| Item::Mul(a, b))
        .parse(input)
}
fn parse_mod(input: &str) -> IResult<&str, Item> {
    (tag("mod "), complete::anychar, tag(" "), parse_target)
        .map(|(_, a, _, b)| Item::Mod(a, b))
        .parse(input)
}
fn parse_rcv(input: &str) -> IResult<&str, Item> {
    preceded(tag("rcv "), complete::anychar)
        .map(Item::Rcv)
        .parse(input)
}
fn parse_jgz(input: &str) -> IResult<&str, Item> {
    (tag("jgz "), parse_target, tag(" "), parse_target)
        .map(|(_, a, _, b)| Item::Jgz(a, b))
        .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Item> {
    alt((
        parse_snd, parse_set, parse_add, parse_mul, parse_mod, parse_rcv, parse_jgz,
    ))
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
