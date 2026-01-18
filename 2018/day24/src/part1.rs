use crate::parsing::{DamageType, Group, Team, parse_input};

pub fn process(_input: &str) -> usize {
    let mut groups = parse_input(_input);

    loop {
        let target_plan = target_selection(&mut groups);
        attacking(&mut groups, target_plan);
        if finish(&groups) {
            break;
        }
    }
    groups.into_iter().map(|g| g.stats.units).sum()
}

fn target_selection(groups: &mut Vec<Group>) -> Vec<Option<usize>> {
    groups.sort_unstable_by(|a, b| {
        (b.effective_power(), b.stats.initiative).cmp(&(a.effective_power(), a.stats.initiative))
    });
    let mut ans = vec![];
    let mut chosen_def_groups: Vec<usize> = vec![];
    for a in groups.iter() {
        let mut max_damage = usize::MIN;
        let mut chosen = None;
        for (i, b) in groups
            .iter()
            .enumerate()
            .filter(|(i, b)| Team::is_enemy(a, b) && !chosen_def_groups.contains(&i))
        {
            let dealt = deal_damage(a, b);
            if dealt > max_damage {
                max_damage = dealt;
                chosen = Some(i);
            }
        }
        if let Some(i) = chosen.clone() {
            chosen_def_groups.push(i);
        }
        ans.push(chosen);
    }

    ans
}

fn attacking(groups: &mut Vec<Group>, plan: Vec<Option<usize>>) {
    let mut orders: Vec<(usize, Option<usize>)> = plan.into_iter().enumerate().collect();
    let mut dead_list = vec![];

    orders.sort_unstable_by(|(i, a), (j, b)| {
        groups
            .get(*j)
            .unwrap()
            .stats
            .initiative
            .cmp(&groups.get(*i).unwrap().stats.initiative)
    });
    for (i, p) in orders {
        if let Some(j) = p {
            let a = groups.get(i).unwrap();
            let b = groups.get(j).unwrap();
            let dealt = deal_damage(a, b);
            let killed = dealt / b.stats.hp;
            match b.stats.units.checked_sub(killed) {
                None | Some(0) => {}
                Some(x) => {
                    groups.get_mut(j).unwrap().stats.units = x;
                }
            }
        }
    }
}

fn finish(groups: &[Group]) -> bool {
    groups.iter().all(|g| matches!(g.team, Team::Immune))
        || groups.iter().all(|g| matches!(g.team, Team::Infection))
}

fn deal_damage(a: &Group, b: &Group) -> usize {
    if b.stats.immune.contains(&a.stats.atk_type) {
        0
    } else if b.stats.weakness.contains(&a.stats.atk_type) {
        a.effective_power() * 2
    } else {
        a.effective_power()
    }
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
        assert_eq!(process(fixture), 5216);
    }
}
