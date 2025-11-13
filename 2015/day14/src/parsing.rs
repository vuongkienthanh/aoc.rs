use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
};
fn parse_line(input: &str) -> IResult<&str, (usize, usize, usize)> {
    (
        alpha1,
        tag(" can fly "),
        map_res(digit1, str::parse),
        tag(" km/s for "),
        map_res(digit1, str::parse),
        tag(" seconds, but then must rest for "),
        map_res(digit1, str::parse),
        tag(" seconds."),
    )
        .map(|(_, _, km, _, t1, _, t2, _)| (km, t1, t2))
        .parse(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<(usize, usize, usize)>> {
    separated_list1(line_ending, parse_line).parse(input)
}
