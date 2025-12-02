use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, line_ending, self},
    multi::separated_list1,
    sequence::preceded,
};

fn parse_line(input: &str) -> IResult<&str, isize> {
    alt((
        preceded(char('L'), complete::isize).map(|x| -x),
        preceded(char('R'), complete::isize),
    ))
    .parse(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<isize>> {
    separated_list1(line_ending, parse_line).parse(input)
}
