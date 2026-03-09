use fxhash::FxHashMap;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::preceded,
};

fn parse_line(input: &str) -> IResult<&str, (&str, usize, Vec<&str>)> {
    (
        preceded(tag("Valve "), alpha1),
        preceded(tag(" has flow rate="), complete::usize),
        preceded(
            (
                alt((tag("; tunnels lead to "), tag("; tunnel leads to "))),
                alt((tag("valves "), tag("valve "))),
            ),
            separated_list1(tag(", "), alpha1),
        ),
    )
        .parse(input)
}

pub fn parse_input(input: &str) -> FxHashMap<&str, (usize, Vec<&str>)> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
        .into_iter()
        .map(|(from, rate, to)| (from, (rate, to)))
        .collect()
}

pub fn encode(
    input: FxHashMap<&str, (usize, Vec<&str>)>,
) -> (FxHashMap<u64, (usize, Vec<u64>)>, u64) {
    let map: FxHashMap<&str, u64> = input
        .keys()
        .enumerate()
        .map(|(i, k)| (*k, i as u64))
        .collect();
    (
        input
            .into_iter()
            .map(|(k, (u, v))| {
                (
                    map.get(&k).cloned().unwrap(),
                    (
                        u,
                        v.into_iter()
                            .map(|x| map.get(&x).cloned().unwrap())
                            .collect(),
                    ),
                )
            })
            .collect(),
        map.get("AA").cloned().unwrap(),
    )
}
