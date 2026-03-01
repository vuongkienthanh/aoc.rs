use nom::{
    IResult, Parser,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};

fn parse_elf(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(line_ending, complete::usize).parse(input)
}

pub fn parse_input(input: &str) -> Vec<Vec<usize>> {
    all_consuming(separated_list1((line_ending, line_ending), parse_elf))
        .parse(input)
        .unwrap()
        .1
}
