use nom::{
    IResult, Parser,
    bytes::complete::{tag, take, take_while},
    character::complete::{self, line_ending, space1},
    combinator::all_consuming,
    multi::{count, separated_list1},
    sequence::{preceded, separated_pair, terminated},
};

type Size = (usize, usize);
type Item = (Size, [usize; 6]);

fn line(input: &str) -> IResult<&str, usize> {
    terminated(
        take_while(|x| x == '.' || x == '#').map(|x: &str| x.chars().filter(|x| *x == '#').count()),
        line_ending,
    )
    .parse(input)
}
fn parse_block(input: &str) -> IResult<&str, usize> {
    preceded((take(2usize), line_ending), count(line, 3))
        .map(|v| v.into_iter().sum())
        .parse(input)
}
fn parse_blocks(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(line_ending, parse_block).parse(input)
}
fn parse_line(input: &str) -> IResult<&str, Item> {
    (
        terminated(
            separated_pair(complete::usize, complete::char('x'), complete::usize),
            tag(": "),
        ),
        separated_list1(space1, complete::usize).map(|v| [v[0], v[1], v[2], v[3], v[4], v[5]]),
    )
        .parse(input)
}

pub fn parse_input(input: &str) -> (Vec<usize>, Vec<Item>) {
    all_consuming(separated_pair(
        parse_blocks,
        line_ending,
        separated_list1(line_ending, parse_line),
    ))
    .parse(input)
    .unwrap()
    .1
}
