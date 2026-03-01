use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};

#[derive(Debug)]
pub enum Group {
    Div1(isize),
    Div26(isize),
}

fn parse_group(input: &str) -> IResult<&str, Group> {
    (
        tag("inp w\nmul x 0\nadd x z\nmod x 26\ndiv z "),
        complete::isize,
        tag("\nadd x "),
        complete::isize,
        tag("\neql x w\neql x 0\nmul y 0\nadd y 25\nmul y x\nadd y 1\nmul z y\nmul y 0\nadd y w\nadd y "),
        complete::isize,
        tag("\nmul y x\nadd z y"),
    )
        .map(|(_, a, _, b, _, c, _)| match a {
            1 => Group::Div1(c),
            26 => Group::Div26(b),
            _ => panic!(),
        })
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Group> {
    all_consuming(separated_list1(line_ending, parse_group))
        .parse(input)
        .unwrap()
        .1
}
