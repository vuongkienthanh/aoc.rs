use crate::parsing::parse_input;
use crate::{State, branch_and_bound};

pub fn process(_input: &str) -> u32 {
    parse_input(_input)
        .into_iter()
        .map(|blueprint| {
            let mut best = 0;
            branch_and_bound(&blueprint, State::new(24), &mut best);
            blueprint.id as u32 * best as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 33);
    }
}
