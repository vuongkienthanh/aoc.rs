use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};

#[derive(Debug)]
pub enum Target {
    Register(usize),
    Value(isize),
}

#[derive(Debug)]
pub enum Item {
    Set(usize, Target),
    Sub(usize, Target),
    Mul(usize, Target),
    Jnz(Target, Target),
}

fn parse_register(input: &str) -> IResult<&str, usize> {
    complete::anychar
        .map(|x| (x as u8 - b'a') as usize)
        .parse(input)
}

fn parse_target(input: &str) -> IResult<&str, Target> {
    alt((
        complete::isize.map(Target::Value),
        parse_register.map(Target::Register),
    ))
    .parse(input)
}

fn parse_set(input: &str) -> IResult<&str, Item> {
    (tag("set "), parse_register, tag(" "), parse_target)
        .map(|(_, a, _, b)| Item::Set(a, b))
        .parse(input)
}
fn parse_sub(input: &str) -> IResult<&str, Item> {
    (tag("sub "), parse_register, tag(" "), parse_target)
        .map(|(_, a, _, b)| Item::Sub(a, b))
        .parse(input)
}
fn parse_mul(input: &str) -> IResult<&str, Item> {
    (tag("mul "), parse_register, tag(" "), parse_target)
        .map(|(_, a, _, b)| Item::Mul(a, b))
        .parse(input)
}
fn parse_jnz(input: &str) -> IResult<&str, Item> {
    (tag("jnz "), parse_target, tag(" "), parse_target)
        .map(|(_, a, _, b)| Item::Jnz(a, b))
        .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Item> {
    alt((parse_set, parse_sub, parse_mul, parse_jnz)).parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
