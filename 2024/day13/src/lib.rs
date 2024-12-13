pub mod part1;
pub mod part2;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

type Coord = (isize, isize);

fn parse(input: &str) -> IResult<&str, Vec<(Coord, Coord, Coord)>> {
    separated_list1(line_ending, group)(input)
}

fn button(input: &str) -> IResult<&str, Coord> {
    preceded(
        alt((tag("Button A: "), tag("Button B: "))),
        separated_pair(
            preceded(tag("X+"), digit1.map(|x: &str| x.parse::<isize>().unwrap())),
            tag(", "),
            preceded(tag("Y+"), digit1.map(|x: &str| x.parse::<isize>().unwrap())),
        ),
    )(input)
}

fn prize(input: &str) -> IResult<&str, Coord> {
    preceded(
        tag("Prize: "),
        separated_pair(
            preceded(tag("X="), digit1.map(|x: &str| x.parse::<isize>().unwrap())),
            tag(", "),
            preceded(tag("Y="), digit1.map(|x: &str| x.parse::<isize>().unwrap())),
        ),
    )(input)
}

fn group(input: &str) -> IResult<&str, (Coord, Coord, Coord)> {
    tuple((
        terminated(button, line_ending),
        terminated(button, line_ending),
        terminated(prize, line_ending),
    ))(input)
}

fn find_token(a: Coord, b: Coord, p: Coord) -> Option<isize> {
    let a_ = a.0 + a.1;
    let b_ = b.0 + b.1;
    let p_ = p.0 + p.1;

    let b_times = (p.0 * a_ - a.0 * p_) / (b.0 * a_ - a.0 * b_);

    let a_times = (p_ - b_ * b_times) / a_;
    (a.0 * a_times + b.0 * b_times == p.0 && a.1 * a_times + b.1 * b_times == p.1)
        .then_some(a_times * 3 + b_times)
}
