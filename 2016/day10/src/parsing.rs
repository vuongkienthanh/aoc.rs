use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::preceded,
};

#[derive(Debug)]
pub enum Input {
    Init(usize, usize),
    Bot(usize, Target, Target),
}
#[derive(Clone, Debug, Default)]
pub enum Target {
    #[default]
    Unknown,
    Output(usize),
    Bot(usize),
}
type Item = Input;

fn parse_init_value(input: &str) -> IResult<&str, Input> {
    (
        tag("value "),
        complete::usize,
        tag(" goes to bot "),
        complete::usize,
    )
        .map(|(_, a, _, b)| Input::Init(a, b))
        .parse(input)
}
fn parse_bot_action(input: &str) -> IResult<&str, Input> {
    (
        tag("bot "),
        complete::usize,
        parse_low_target,
        parse_high_target,
    )
        .map(|(_, a, b, c)| Input::Bot(a, b, c))
        .parse(input)
}

fn parse_low_target(input: &str) -> IResult<&str, Target> {
    alt((
        preceded(tag(" gives low to output "), complete::usize).map(Target::Output),
        preceded(tag(" gives low to bot "), complete::usize).map(Target::Bot),
    ))
    .parse(input)
}
fn parse_high_target(input: &str) -> IResult<&str, Target> {
    alt((
        preceded(tag(" and high to output "), complete::usize).map(Target::Output),
        preceded(tag(" and high to bot "), complete::usize).map(Target::Bot),
    ))
    .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Item> {
    alt((parse_init_value, parse_bot_action)).parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
