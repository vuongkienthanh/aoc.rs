use grid::Grid;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::delimited,
};

type Item = (usize, Grid<char>);

fn parse_tile(input: &str) -> IResult<&str, Item> {
    (
        delimited(tag("Tile "), complete::usize, tag(":\n")),
        separated_list1(
            line_ending,
            many1(alt((complete::char('.'), complete::char('#')))),
        )
        .map(|g| Grid::from(g)),
    )
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1((line_ending, line_ending), parse_tile))
        .parse(input)
        .unwrap()
        .1
}
