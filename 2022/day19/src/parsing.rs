use crate::{Blueprint, ONE_CLAY, ONE_OBSIDIAN, ONE_ORE};
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
};

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, id) = delimited(tag("Blueprint "), complete::u8, tag(": ")).parse(input)?;
    let (input, ore_robot_cost) =
        delimited(tag("Each ore robot costs "), complete::u8, tag(" ore. "))
            .map(|ore| ONE_ORE * ore)
            .parse(input)?;
    let (input, clay_robot_cost) =
        delimited(tag("Each clay robot costs "), complete::u8, tag(" ore. "))
            .map(|ore| ONE_ORE * ore)
            .parse(input)?;
    let (input, obsidian_robot_cost) = delimited(
        tag("Each obsidian robot costs "),
        separated_pair(complete::u8, tag(" ore and "), complete::u8),
        tag(" clay. "),
    )
    .map(|(ore, clay)| ONE_ORE * ore + ONE_CLAY * clay)
    .parse(input)?;
    let (input, geode_robot_cost) = delimited(
        tag("Each geode robot costs "),
        separated_pair(complete::u8, tag(" ore and "), complete::u8),
        tag(" obsidian."),
    )
    .map(|(ore, obsidian)| ONE_ORE * ore + ONE_OBSIDIAN * obsidian)
    .parse(input)?;
    Ok((
        input,
        Blueprint {
            id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        },
    ))
}

pub fn parse_input(input: &str) -> Vec<Blueprint> {
    all_consuming(separated_list1(line_ending, parse_blueprint))
        .parse(input)
        .unwrap()
        .1
}
