use nom::{
    IResult, Parser, character::complete, combinator::all_consuming, multi::separated_list1,
};

type Item = usize;

fn parse_line(input: &str) -> IResult<&str, Vec<Item>> {
    separated_list1(complete::char(','), complete::usize).parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(parse_line).parse(input).unwrap().1
}
