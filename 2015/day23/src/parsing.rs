use aoc_helper::nom::parse_integer;
use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag, character::complete::line_ending,
    multi::separated_list1, sequence::preceded,
};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum CMD {
    hlf(usize),
    tpl(usize),
    inc(usize),
    jmp(isize),
    jie(usize, isize),
    jio(usize, isize),
}

fn parse_reg(input: &str) -> IResult<&str, usize> {
    alt((tag("a").map(|_| 0), tag("b").map(|_| 1))).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, CMD> {
    alt((
        preceded(tag("hlf "), parse_reg).map(|x| CMD::hlf(x)),
        preceded(tag("tpl "), parse_reg).map(|x| CMD::tpl(x)),
        preceded(tag("inc "), parse_reg).map(|x| CMD::inc(x)),
        preceded(tag("jmp "), parse_integer).map(|x| CMD::jmp(x)),
        (tag("jie "), parse_reg, tag(", "), parse_integer).map(|(_, r, _, x)| CMD::jie(r, x)),
        (tag("jio "), parse_reg, tag(", "), parse_integer).map(|(_, r, _, x)| CMD::jio(r, x)),
    ))
    .parse(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<CMD>> {
    separated_list1(line_ending, parse_line).parse(input)
}
