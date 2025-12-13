#[allow(unused_imports)]
// use aoc_helper::nom::parse_signed_usize;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take, take_until, take_while},
    character::complete::{self, alpha1, char, line_ending},
    combinator::{all_consuming, peek},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
};
use nom_locate::{LocatedSpan, position};

type Span<'a> = LocatedSpan<&'a str>;

pub fn parse_space(input: Span) -> IResult<Span, Span> {
    take_while(|x| x == '.').parse(input)
}

pub fn parse_start(input: Span) -> IResult<Span, usize> {
    let (input, _) = parse_space.parse(input)?;
    let (input, start) = position(input)?;
    let (input, _) = (
        char('S'),
        parse_space,
        line_ending,
        parse_space,
        line_ending,
    )
        .parse(input)?;
    Ok((input, start.get_column()))
}

pub fn parse_splitters(input: Span) -> IResult<Span, Vec<usize>> {
    // let (input, current_line) = take_while(|x| x == '.' || x == '^').parse(input)?;
    // let (mut current_line, _) = parse_space.parse(current_line)?;
    // let mut ans = vec![];
    // while !current_line.is_empty() {
    //     let (rest_of_line, pos) = position(current_line)?;
    //     ans.push(pos.get_column());
    //     let (rest_of_line, _) = char('^').parse(rest_of_line)?;
    //     let (rest_of_line, _) = parse_space.parse(rest_of_line)?;
    //     current_line = rest_of_line;
    // }
    // let (input, _) = (line_ending, parse_space).parse(input)?;
    // Ok((input, ans))
    // let (mut input, _) = parse_space.parse(current_line)?;
    let mut ans = vec![];
    let (mut input, _) = parse_space(input)?;
    loop {
        let (_, c) = peek(take(1usize)).parse(input)?;
        match *c {
            "^" => {
                let (_, pos) = position(input)?;
                ans.push(pos.get_column());
                input = take(1usize).parse(input)?.0;
                input = parse_space(input)?.0;
            }
            _ => break,
        }
    }
    let (input, _) = (line_ending, parse_space).parse(input)?;
    Ok((input, ans))
}

pub fn parse_input(input: &str) -> (usize, Vec<Vec<usize>>) {
    all_consuming((parse_start, separated_list1(line_ending, parse_splitters)))
        .parse(Span::new(input))
        .unwrap()
        .1
}
