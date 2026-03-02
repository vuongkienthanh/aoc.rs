use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
};

fn parse_crate(input: &str) -> IResult<&str, Option<char>> {
    alt((
        delimited(tag("["), complete::anychar, tag("]")).map(|x| Some(x)),
        tag("   ").map(|_| None),
    ))
    .parse(input)
}
fn parse_crate_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(tag(" "), parse_crate).parse(input)
}
fn parse_crates(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(line_ending, parse_crate_line)
        .map(|v| {
            let cols = v.last().unwrap().len();
            let mut ans = vec![vec![]; cols];
            for row in v.into_iter().rev() {
                for (i, c) in row.into_iter().enumerate() {
                    if let Some(c) = c {
                        ans[i].push(c);
                    }
                }
            }
            ans
        })
        .parse(input)
}

fn parse_instruction_line(input: &str) -> IResult<&str, (usize, usize, usize)> {
    (
        preceded(tag("move "), complete::usize),
        preceded(tag(" from "), complete::usize),
        preceded(tag(" to "), complete::usize),
    )
        .parse(input)
}
fn parse_instructions(input: &str) -> IResult<&str, Vec<(usize, usize, usize)>> {
    separated_list1(line_ending, parse_instruction_line).parse(input)
}
pub fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    all_consuming(separated_pair(
        parse_crates,
        (
            line_ending,
            take_while(|x| x != '\n'),
            line_ending,
            line_ending,
        ),
        parse_instructions,
    ))
    .parse(input)
    .unwrap()
    .1
}