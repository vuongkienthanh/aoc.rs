use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
};

#[derive(Debug, Eq, PartialEq)]
pub enum DamageType {
    Slashing,
    Radiation,
    Fire,
    Bludgeoning,
    Cold,
}

#[derive(Debug)]
pub struct Item {
    pub units: usize,
    pub hp: usize,
    pub weakness: Vec<DamageType>,
    pub immune: Vec<DamageType>,
    pub atk: usize,
    pub atk_type: DamageType,
    pub initiative: usize,
}

#[derive(Debug)]
pub enum Team {
    Immune,
    Infection,
}
impl Team {
    pub fn is_enemy(a: &Group, b: &Group) -> bool {
        matches!(
            (&a.team, &b.team),
            (Team::Immune, Team::Infection) | (Team::Infection, Team::Immune),
        )
    }
}

#[derive(Debug)]
pub struct Group {
    pub team: Team,
    pub is_alive: bool,
    pub stats: Item,
}
impl Group {
    pub fn effective_power(&self) -> usize {
        self.stats.units * self.stats.atk
    }
}

fn parse_damage_type(input: &str) -> IResult<&str, DamageType> {
    alt((
        tag("fire").map(|_| DamageType::Fire),
        tag("cold").map(|_| DamageType::Cold),
        tag("radiation").map(|_| DamageType::Radiation),
        tag("slashing").map(|_| DamageType::Slashing),
        tag("bludgeoning").map(|_| DamageType::Bludgeoning),
    ))
    .parse(input)
}
fn parse_damage_types(input: &str) -> IResult<&str, Vec<DamageType>> {
    separated_list1(tag(", "), parse_damage_type).parse(input)
}

fn parse_immune(input: &str) -> IResult<&str, Vec<DamageType>> {
    preceded(tag("immune to "), parse_damage_types).parse(input)
}
fn parse_weakness(input: &str) -> IResult<&str, Vec<DamageType>> {
    preceded(tag("weak to "), parse_damage_types).parse(input)
}
fn parse_weakness_immune(input: &str) -> IResult<&str, (Vec<DamageType>, Vec<DamageType>)> {
    alt((
        delimited(
            tag("("),
            separated_pair(parse_weakness, tag("; "), parse_immune),
            tag(") "),
        ),
        delimited(
            tag("("),
            separated_pair(parse_immune, tag("; "), parse_weakness).map(|(a, b)| (b, a)),
            tag(") "),
        ),
        delimited(tag("("), parse_weakness.map(|w| (w, vec![])), tag(") ")),
        delimited(tag("("), parse_immune.map(|i| (vec![], i)), tag(") ")),
        take(0usize).map(|_| (vec![], vec![])),
    ))
    .parse(input)
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    ((
        complete::usize,
        tag(" units each with "),
        complete::usize,
        tag(" hit points "),
        parse_weakness_immune,
        tag("with an attack that does "),
        complete::usize,
        tag(" "),
        parse_damage_type,
        tag(" damage at initiative "),
        complete::usize,
    ))
        .map(
            |(units, _, hp, _, (weakness, immune), _, atk, _, atk_type, _, initiative)| Item {
                units,
                hp,
                weakness,
                immune,
                atk,
                atk_type,
                initiative,
            },
        )
        .parse(input)
}

fn parse_immune_system(input: &str) -> IResult<&str, Vec<Group>> {
    preceded(
        (tag("Immune System:"), line_ending),
        separated_list1(
            line_ending,
            parse_item.map(|item| Group {
                team: Team::Immune,
                stats: item,
            }),
        ),
    )
    .parse(input)
}

fn parse_infection(input: &str) -> IResult<&str, Vec<Group>> {
    preceded(
        (tag("Infection:"), line_ending),
        separated_list1(
            line_ending,
            parse_item.map(|item| Group {
                team: Team::Infection,
                stats: item,
            }),
        ),
    )
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Group> {
    let (mut team1, team2) = all_consuming(separated_pair(
        parse_immune_system,
        (line_ending, line_ending),
        parse_infection,
    ))
    .parse(input)
    .unwrap()
    .1;
    team1.extend(team2);
    team1
}
