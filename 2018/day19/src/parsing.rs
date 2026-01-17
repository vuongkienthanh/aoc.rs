use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    addr,
    addi,
    mulr,
    muli,
    banr,
    bani,
    borr,
    bori,
    setr,
    seti,
    gtir,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr,
}
fn parse_declaration(input: &str) -> IResult<&str, usize> {
    preceded(tag("#ip "), complete::usize).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, (Opcode, usize, usize, usize)> {
    (
        alt((
            tag("addr ").map(|_| Opcode::addr),
            tag("addi ").map(|_| Opcode::addi),
            tag("mulr ").map(|_| Opcode::mulr),
            tag("muli ").map(|_| Opcode::muli),
            tag("banr ").map(|_| Opcode::banr),
            tag("bani ").map(|_| Opcode::bani),
            tag("borr ").map(|_| Opcode::borr),
            tag("bori ").map(|_| Opcode::bori),
            tag("setr ").map(|_| Opcode::setr),
            tag("seti ").map(|_| Opcode::seti),
            tag("gtir ").map(|_| Opcode::gtir),
            tag("gtri ").map(|_| Opcode::gtri),
            tag("gtrr ").map(|_| Opcode::gtrr),
            tag("eqir ").map(|_| Opcode::eqir),
            tag("eqri ").map(|_| Opcode::eqri),
            tag("eqrr ").map(|_| Opcode::eqrr),
        )),
        complete::usize,
        tag(" "),
        complete::usize,
        tag(" "),
        complete::usize,
    )
        .map(|(a, b, _, c, _, d)| (a, b, c, d))
        .parse(input)
}

pub fn parse_input(input: &str) -> (usize, Vec<(Opcode, usize, usize, usize)>) {
    all_consuming(separated_pair(
        parse_declaration,
        line_ending,
        separated_list1(line_ending, parse_line),
    ))
    .parse(input)
    .unwrap()
    .1
}
