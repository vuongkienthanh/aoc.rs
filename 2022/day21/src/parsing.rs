use fxhash::FxHashMap;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
};

pub type Map<'a> = FxHashMap<&'a str, usize>;

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Operand<'a> {
    Value(usize),
    Name(&'a str),
}

pub enum Yell<'a> {
    Math(Op, Operand<'a>, Operand<'a>),
    Number(usize),
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    alt((
        tag(" + ").map(|_| Op::Add),
        tag(" - ").map(|_| Op::Sub),
        tag(" * ").map(|_| Op::Mul),
        tag(" / ").map(|_| Op::Div),
    ))
    .parse(input)
}

fn parse_operand<'a>(input: &'a str) -> IResult<&'a str, Operand<'a>> {
    alt((
        alpha1.map(Operand::Name),
        complete::usize.map(Operand::Value),
    ))
    .parse(input)
}

fn parse_line<'a>(input: &'a str) -> IResult<&'a str, (&'a str, Yell<'a>)> {
    alt((
        separated_pair(alpha1, tag(": "), complete::usize).map(|(a, b)| (a, Yell::Number(b))),
        (alpha1, tag(": "), parse_operand, parse_op, parse_operand)
            .map(|(a, _, b, op, c)| (a, Yell::Math(op, b, c))),
    ))
    .parse(input)
}

pub fn parse_input<'a>(input: &'a str) -> (Map<'a>, Vec<(&'a str, Op, Operand<'a>, Operand<'a>)>) {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
        .into_iter()
        .fold(
            (FxHashMap::default(), vec![]),
            |(mut hm, mut v), (monkey, yell)| {
                match yell {
                    Yell::Number(x) => {
                        hm.insert(monkey, x);
                    }
                    Yell::Math(o, a, b) => v.push((monkey, o, a, b)),
                }
                (hm, v)
            },
        )
}
