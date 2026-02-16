use fxhash::FxHashSet;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

#[derive(Debug)]
pub enum F {
    X(usize),
    Y(usize),
}

fn parse_dots(input: &str) -> IResult<&str, FxHashSet<(usize, usize)>> {
    separated_list1(
        line_ending,
        separated_pair(complete::usize, tag(","), complete::usize),
    )
    .map(|v| v.into_iter().collect())
    .parse(input)
}
fn parse_folds(input: &str) -> IResult<&str, Vec<F>> {
    separated_list1(
        line_ending,
        alt((
            preceded(tag("fold along x="), complete::usize).map(F::X),
            preceded(tag("fold along y="), complete::usize).map(F::Y),
        )),
    )
    .parse(input)
}

pub fn parse_input(input: &str) -> (FxHashSet<(usize, usize)>, Vec<F>) {
    all_consuming(separated_pair(
        parse_dots,
        (line_ending, line_ending),
        parse_folds,
    ))
    .parse(input)
    .unwrap()
    .1
}
