use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    (
        tag("Blueprint "),
        complete::usize,
        tag(": Each ore robot costs "),
        complete::usize,
        tag(" ore. Each clay robot costs "),
        complete::usize,
        tag(" ore. Each obsidian robot costs "),
        complete::usize,
        tag(" ore and "),
        complete::usize,
        tag(" clay. Each geode robot costs "),
        complete::usize,
        tag(" ore and "),
        complete::usize,
        tag(" obsidian."),
    )
        .map(|(_, i, _, a, _, b, _, c, _, d, _, e, _, f, _)| Blueprint {
            id: i,
            ore_robot_need: a,
            clay_robot_need: b,
            obsidian_robot_need: (c, d),
            geode_robot_need: (e, f),
        })
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Blueprint> {
    all_consuming(separated_list1(line_ending, parse_blueprint))
        .parse(input)
        .unwrap()
        .1
}