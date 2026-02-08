pub mod part1;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take, take_while},
    character::complete::{alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated},
};

fn parse_location(input: &str) -> IResult<&str, String> {
    delimited(
        tag("\n\n\n== "),
        take_while(|x| x != '=').map(|x: &str| x[..x.len() - 1].to_string()),
        (tag("==\n"), take_while(|x| x != '\n'), tag("\n\n")),
    )
    .parse(input)
}
fn parse_doors(input: &str) -> IResult<&str, Vec<String>> {
    delimited(
        tag("Doors here lead:\n"),
        separated_list1(
            line_ending,
            preceded(tag("- "), alpha1.map(|x: &str| x.to_string())),
        ),
        tag("\n\n"),
    )
    .parse(input)
}
fn parse_item(input: &str) -> IResult<&str, String> {
    alt((
        delimited(
            tag("Items here:\n"),
            preceded(
                tag("- "),
                take_while(|x| x != '\n').map(|x: &str| x.to_string()),
            ),
            tag("\n\n"),
        ),
        take(0usize).map(|_| String::new()),
    ))
    .parse(input)
}
fn parse_block(input: &str) -> (String, Vec<String>, String) {
    all_consuming(terminated(
        (parse_location, parse_doors, parse_item),
        tag("Command?\n"),
    ))
    .parse(input)
    .unwrap()
    .1
}

fn _parse_inventory(input: &str) -> IResult<&str, Vec<String>> {
    alt((
        delimited(
            tag("\nItems in your inventory:\n"),
            separated_list1(
                line_ending,
                preceded(
                    tag("- "),
                    take_while(|x| x != '\n').map(|x: &str| x.to_string()),
                ),
            ),
            tag("\n\nCommand?\n"),
        ),
        tag("\nYou aren't carrying any items.\n\nCommand?\n").map(|_| vec![]),
    ))
    .parse(input)
}
fn parse_inventory(input: &str) -> Vec<String> {
    all_consuming(_parse_inventory).parse(input).unwrap().1
}
