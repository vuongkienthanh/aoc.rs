use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::preceded,
};
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum CMD {
    hlf(usize),
    tpl(usize),
    inc(usize),
    jmpf(usize),
    jmpb(usize),
    jief(usize, usize),
    jieb(usize, usize),
    jiof(usize, usize),
    jiob(usize, usize),
}

fn parse_reg(input: &str) -> IResult<&str, usize> {
    alt((tag("a").map(|_| 0), tag("b").map(|_| 1))).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, CMD> {
    alt((
        preceded(tag("hlf "), parse_reg).map(CMD::hlf),
        preceded(tag("tpl "), parse_reg).map(CMD::tpl),
        preceded(tag("inc "), parse_reg).map(CMD::inc),
        preceded(tag("jmp +"), complete::usize).map(CMD::jmpf),
        preceded(tag("jmp -"), complete::usize).map(CMD::jmpb),
        (tag("jie "), parse_reg, tag(", +"), complete::usize).map(|(_, r, _, x)| CMD::jief(r, x)),
        (tag("jie "), parse_reg, tag(", -"), complete::usize).map(|(_, r, _, x)| CMD::jieb(r, x)),
        (tag("jio "), parse_reg, tag(", +"), complete::usize).map(|(_, r, _, x)| CMD::jiof(r, x)),
        (tag("jio "), parse_reg, tag(", -"), complete::usize).map(|(_, r, _, x)| CMD::jiob(r, x)),
    ))
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<CMD> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
