pub mod part1;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take, take_while},
    character::complete::{self, alpha1, line_ending, multispace0},
    combinator::all_consuming,
    multi::{count, many1, separated_list1},
    sequence::{delimited, preceded, terminated},
};
// == Corridor ==                                                                                                                                                                              ▐
// The metal walls and the metal floor are slightly different colors. Or are they?                                                                                                             ▐
//                                                                                                                                                                                             ▐
// Doors here lead:                                                                                                                                                                            ▐
// - east                                                                                                                                                                                      ▐
// - south                                                                                                                                                                                     ▐
// - west                                                                                                                                                                                      ▐
//                                                                                                                                                                                             ▐
// Items here:
// - ornament
//
// Command?
//
//

fn parse_location(input: &str) -> IResult<&str, &str> {
    delimited(
        (count(line_ending, 3), tag("== ")),
        take_while(|x| x != '=').map(|x: &str| &x[..x.len() - 1]),
        (
            tag("=="),
            line_ending,
            take_while(|x| x != '\n'),
            line_ending,
            line_ending,
        ),
    )
    .parse(input)
}
fn parse_doors(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        (tag("Doors here lead:"), line_ending),
        separated_list1(line_ending, preceded((tag("- ")), alpha1)),
        (line_ending, line_ending),
    )
    .parse(input)
}
fn parse_items(input: &str) -> IResult<&str, Vec<&str>> {
    alt((
        delimited(
            (tag("Items here:"), line_ending),
            separated_list1(line_ending, preceded(tag("- "), take_while(|x| x != '\n'))),
            (line_ending, line_ending),
        ),
        take(0usize).map(|_| vec![]),
    ))
    .parse(input)
}
fn parse_block(input: &str) -> IResult<&str, (&str, Vec<&str>, Vec<&str>)> {
    terminated(
        (parse_location, parse_doors, parse_items),
        tag("Command?\n"),
    )
    .parse(input)
}
fn parse_output(input: &str) -> Vec<(&str, Vec<&str>, Vec<&str>)> {
    all_consuming(many1(parse_block)).parse(input).unwrap().1
}
