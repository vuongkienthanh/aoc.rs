use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};
use std::collections::HashMap;

pub type Graph<'a> = HashMap<&'a str, HashMap<&'a str, isize>>;

fn parse_line(input: &str) -> IResult<&str, (&str, &str, isize)> {
    (
        alpha1,
        alt((
            tag(" would gain ").map(|_| 1),
            tag(" would lose ").map(|_| -1),
        )),
        complete::isize,
        tag(" happiness units by sitting next to "),
        alpha1,
        tag("."),
    )
        .map(|(n1, would, how_much, _, n2, _)| (n1, n2, would * how_much))
        .parse(input)
}

pub fn parse_input<'a>(input: &'a str) -> Graph<'a> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
        .into_iter()
        .fold(Graph::new(), |mut graph, (n1, n2, how_much)| {
            graph.entry(n1).or_default().insert(n2, how_much);
            graph
        })
}
