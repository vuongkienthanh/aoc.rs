use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, char, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};

#[derive(Debug, Clone, Copy)]
pub enum Target {
    Register(usize),
    Value(isize),
}

pub type Register = usize;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum Item {
    cpy(Target, Register),
    inc(Register),
    dec(Register),
    jnz(Target, Target),
    tgl(Target),
    optimized_add_product(Register, Register, Register),
    optimized_add(Register, Register),
    none,
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
fn parse_target_value(input: &str) -> IResult<&str, Target> {
    complete::isize.map(Target::Value).parse(input)
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
    alt((parse_target_value, parse_target_register)).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Item> {
    alt((
        (tag("cpy "), parse_target, tag(" "), parse_register).map(|(_, a, _, b)| Item::cpy(a, b)),
        (tag("inc "), parse_register).map(|(_, a)| Item::inc(a)),
        (tag("dec "), parse_register).map(|(_, a)| Item::dec(a)),
        (tag("jnz "), parse_target, tag(" "), parse_target).map(|(_, a, _, b)| Item::jnz(a, b)),
        (tag("tgl "), parse_target).map(|(_, a)| Item::tgl(a)),
    ))
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
