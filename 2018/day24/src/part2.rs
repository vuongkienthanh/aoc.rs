use crate::parsing::{Group, Team, parse_input};
use crate::{attack_phase, finish, target_select_phase};

pub fn process(_input: &str) -> usize {
    let groups = parse_input(_input);

    'b: for boost in 1.. {
        let mut boosted = boost_immune(&groups, boost);
        loop {
            let plan = target_select_phase(&mut boosted);
            let current_state = state(&boosted);
            attack_phase(&mut boosted, plan);
            if state(&boosted) == current_state {
                continue 'b;
            }

            match finish(&boosted) {
                Some(Team::Immune) => return boosted.into_iter().map(|g| g.stats.units).sum(),
                Some(Team::Infection) => continue 'b,
                None => (),
            }
        }
    }
    panic!("should have an answer")
}

fn state(groups: &[Group]) -> Vec<usize> {
    groups.iter().map(|g| g.stats.units).collect()
}

fn boost_immune(groups: &[Group], by: usize) -> Vec<Group> {
    groups
        .iter()
        .cloned()
        .map(|mut g| {
            if matches!(g.team, Team::Immune) {
                g.stats.atk += by;
                g
            } else {
                g
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 51);
    }
}
