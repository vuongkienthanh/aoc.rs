use nom::{
    IResult, Parser,
    character::complete::{alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};

fn parse_group(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(line_ending, alpha1).parse(input)
}

pub fn parse_input(input: &str) -> Vec<Vec<&str>> {
    all_consuming(separated_list1((line_ending, line_ending), parse_group))
        .parse(input)
        .unwrap()
        .1
}
