use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
};

pub type Item = ((isize, isize, isize), usize);

fn parse_line(input: &str) -> IResult<&str, Item> {
    separated_pair(
        preceded(
            tag("pos="),
            delimited(
                tag("<"),
                separated_list1(tag(","), complete::isize).map(|v| (v[0], v[1], v[2])),
                tag(">"),
            ),
        ),
        tag(", "),
        preceded(tag("r="), complete::usize),
    )
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
