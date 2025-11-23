use aoc_helper::nom::{Sign, parse_integer, parse_number};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending},
    multi::separated_list1,
};

#[derive(Debug, Clone)]
pub enum Target {
    Register(usize),
    Value(usize),
}

pub type Register = usize;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Item {
    cpy(Target, Register),
    inc(Register),
    dec(Register),
    jnzf(Target, usize),
    jnzb(Target, usize),
}

fn parse_register(input: &str) -> IResult<&str, usize> {
    alt((
        char('a').map(|_| 0),
        char('b').map(|_| 1),
        char('c').map(|_| 2),
        char('d').map(|_| 3),
    ))
    .parse(input)
}

fn parse_target_register(input: &str) -> IResult<&str, Target> {
    alt((
        char('a').map(|_| Target::Register(0)),
        char('b').map(|_| Target::Register(1)),
        char('c').map(|_| Target::Register(2)),
        char('d').map(|_| Target::Register(3)),
    ))
    .parse(input)
}

fn parse_target(input: &str) -> IResult<&str, Target> {
    alt((parse_number.map(Target::Value), parse_target_register)).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Item> {
    alt((
        (tag("cpy "), parse_target, tag(" "), parse_register).map(|(_, a, _, b)| Item::cpy(a, b)),
        (tag("inc "), parse_register).map(|(_, a)| Item::inc(a)),
        (tag("dec "), parse_register).map(|(_, a)| Item::dec(a)),
        (tag("jnz "), parse_target, tag(" "), parse_integer).map(|(_, a, _, (s, b))| match s {
            Sign::Positive => Item::jnzf(a, b),
            Sign::Negative => Item::jnzb(a, b),
        }),
    ))
    .parse(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Item>> {
    separated_list1(line_ending, parse_line).parse(input)
}
