use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};

fn parse_line(input: &str) -> IResult<&str, [isize; 5]> {
    (
        alpha1,
        tag(": capacity "),
        complete::isize,
        tag(", durability "),
        complete::isize,
        tag(", flavor "),
        complete::isize,
        tag(", texture "),
        complete::isize,
        tag(", calories "),
        complete::isize,
    )
        .map(|(_, _, a, _, b, _, c, _, d, _, e)| [a, b, c, d, e])
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<[isize; 5]> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
