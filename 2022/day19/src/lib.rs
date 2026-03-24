pub mod parsing;
pub mod part1;
pub mod part2;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, Default)]
struct Resources {
    ore: u8,
    clay: u8,
    obsidian: u8,
}

const ONE_ORE: Resources = Resources {
    ore: 1,
    clay: 0,
    obsidian: 0,
};
const ONE_CLAY: Resources = Resources {
    ore: 0,
    clay: 1,
    obsidian: 0,
};
const ONE_OBSIDIAN: Resources = Resources {
    ore: 0,
    clay: 0,
    obsidian: 1,
};
impl Resources {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(Self {
            ore: self.ore.checked_sub(rhs.ore)?,
            clay: self.clay.checked_sub(rhs.clay)?,
            obsidian: self.obsidian.checked_sub(rhs.obsidian)?,
        })
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
        }
    }
}

impl Mul<u8> for Resources {
    type Output = Self;

    fn mul(self, other: u8) -> Self {
        Self {
            ore: self.ore * other,
            clay: self.clay * other,
            obsidian: self.obsidian * other,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Blueprint {
    id: u8,
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
}

pub fn branch_and_bound(blueprint: &Blueprint, state: State, best: &mut u8) {
    *best = state.geodes_secured.max(*best);
    for state in state.branch(blueprint) {
        if state.bound(blueprint) > *best {
            branch_and_bound(blueprint, state, best);
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct State {
    minutes_remaining: u8,
    geodes_secured: u8,
    resources: Resources,
    resources_rate: Resources,
}

impl State {
    fn new(minutes_remaining: u8) -> Self {
        Self {
            minutes_remaining,
            geodes_secured: 0,
            resources: Default::default(),
            resources_rate: ONE_ORE,
        }
    }

    fn chose_robot(self, cost: Resources, robot: Resources) -> Option<Self> {
        (1..self.minutes_remaining).rev().zip(0..).find_map(
            |(minutes_remaining, minutes_passed)| {
                let resources = self.resources + self.resources_rate * minutes_passed;
                resources.checked_sub(cost).map(|resources| Self {
                    minutes_remaining,
                    resources: resources + self.resources_rate,
                    resources_rate: self.resources_rate + robot,
                    ..self
                })
            },
        )
    }

    fn branch(self, blueprint: &Blueprint) -> impl Iterator<Item = Self> + '_ {
        let max_ore_cost = blueprint
            .clay_robot_cost
            .ore
            .max(blueprint.obsidian_robot_cost.ore)
            .max(blueprint.geode_robot_cost.ore);
        let ore_robot_viable = self.resources_rate.ore < max_ore_cost;
        let clay_robot_viable = self.resources_rate.clay < blueprint.obsidian_robot_cost.clay;
        let obsidian_robot_viable = self.resources_rate.obsidian
            < blueprint.geode_robot_cost.obsidian
            && self.resources_rate.clay > 0;
        let geode_robot_viable = self.resources_rate.obsidian > 0;
        [
            ore_robot_viable.then(|| self.chose_robot(blueprint.ore_robot_cost, ONE_ORE)),
            clay_robot_viable.then(|| self.chose_robot(blueprint.clay_robot_cost, ONE_CLAY)),
            obsidian_robot_viable
                .then(|| self.chose_robot(blueprint.obsidian_robot_cost, ONE_OBSIDIAN)),
            geode_robot_viable.then(|| {
                self.chose_robot(blueprint.geode_robot_cost, Default::default())
                    .map(|state| Self {
                        geodes_secured: state.geodes_secured + state.minutes_remaining,
                        ..state
                    })
            }),
        ]
        .into_iter()
        .flatten()
        .flatten()
    }

    // we have unlimited ore and clay, and prefer building geode robots when possible
    fn bound(self, blueprint: &Blueprint) -> u8 {
        let geode_cost = blueprint.geode_robot_cost.obsidian;
        let (_, _, geodes) = (0..self.minutes_remaining).rev().fold(
            (
                self.resources.obsidian,
                self.resources_rate.obsidian,
                self.geodes_secured,
            ),
            |(obsidian, rate, geodes), minutes_remaining| {
                if obsidian >= geode_cost {
                    (
                        obsidian + rate - geode_cost,
                        rate,
                        geodes.saturating_add(minutes_remaining),
                    )
                } else {
                    (obsidian + rate, rate + 1, geodes)
                }
            },
        );
        geodes
    }
}
