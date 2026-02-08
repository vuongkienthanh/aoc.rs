use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded},
};

#[derive(Debug)]
pub struct Particle {
    pub p: [isize; 3],
    pub v: [isize; 3],
    pub a: [isize; 3],
}

fn parse_triplet(input: &str) -> IResult<&str, [isize; 3]> {
    delimited(
        tag("<"),
        (
            complete::isize,
            tag(","),
            complete::isize,
            tag(","),
            complete::isize,
        )
            .map(|(a, _, b, _, c)| [a, b, c]),
        tag(">"),
    )
    .parse(input)
}

fn parse_p(input: &str) -> IResult<&str, [isize; 3]> {
    preceded(tag("p="), parse_triplet).parse(input)
}
fn parse_v(input: &str) -> IResult<&str, [isize; 3]> {
    preceded(tag("v="), parse_triplet).parse(input)
}
fn parse_a(input: &str) -> IResult<&str, [isize; 3]> {
    preceded(tag("a="), parse_triplet).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Particle> {
    (parse_p, tag(", "), parse_v, tag(", "), parse_a)
        .map(|(p, _, v, _, a)| Particle { p, v, a })
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Particle> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
