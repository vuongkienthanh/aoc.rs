use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};
fn parse_line(input: &str) -> IResult<&str, (usize, usize, usize)> {
    (
        alpha1,
        tag(" can fly "),
        complete::usize,
        tag(" km/s for "),
        complete::usize,
        tag(" seconds, but then must rest for "),
        complete::usize,
        tag(" seconds."),
    )
        .map(|(_, _, km, _, t1, _, t2, _)| (km, t1, t2))
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<(usize, usize, usize)> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
