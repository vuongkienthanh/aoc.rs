use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
};
use std::collections::HashMap;

pub type Graph<'a> = HashMap<&'a str, HashMap<&'a str, isize>>;

fn parse_line(input: &str) -> IResult<&str, (&str, &str, isize)> {
    let (rest, (n1, would, how_much, _, n2, _)) = (
        alpha1,
        alt((
            tag(" would gain ").map(|_| 1),
            tag(" would lose ").map(|_| -1),
        )),
        map_res(digit1, str::parse::<isize>),
        tag(" happiness units by sitting next to "),
        alpha1,
        tag("."),
    )
        .parse(input)?;
    Ok((rest, (n1, n2, would * how_much)))
}

pub fn parse_input<'a>(input: &'a str) -> IResult<&'a str, Graph<'a>> {
    let (rest, v) = separated_list1(line_ending, parse_line).parse(input)?;
    let mut graph = Graph::new();
    for (n1, n2, how_much) in v {
        graph.entry(n1).or_default().insert(n2, how_much);
    }

    Ok((rest, graph))
}
