pub mod parsing;
pub mod part1;
pub mod part2;
use parsing::{Group, Team};

fn target_select_phase(groups: &mut [Group]) -> Vec<Option<usize>> {
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
            .filter(|(i, b)| a.is_enemy_of(b) && !chosen_def_groups.contains(i))
        {
            let damage = a.would_deal(b);
            if damage > max_damage {
                max_damage = damage;
                chosen = Some(i);
            }
        }
        if let Some(i) = chosen {
            chosen_def_groups.push(i);
        }
        ans.push(chosen);
    }

    ans
}

fn attack_phase(groups: &mut Vec<Group>, plan: Vec<Option<usize>>) {
    let mut orders: Vec<(usize, Option<usize>)> = plan.into_iter().enumerate().collect();

    orders.sort_unstable_by(|(i, _), (j, _)| {
        groups
            .get(*j)
            .unwrap()
            .stats
            .initiative
            .cmp(&groups.get(*i).unwrap().stats.initiative)
    });
    for (i, p) in orders {
        let a = groups.get(i).unwrap();
        if a.is_alive()
            && let Some(j) = p
        {
            let b = groups.get(j).unwrap();
            let damage = a.would_deal(b);
            let units_killed = damage / b.stats.hp;
            groups.get_mut(j).unwrap().stats.units = b.stats.units.saturating_sub(units_killed);
        }
    }
    groups.retain(|g| g.is_alive());
}

fn finish(groups: &[Group]) -> Option<Team> {
    if groups.iter().all(|g| matches!(g.team, Team::Immune)) {
        Some(Team::Immune)
    } else if groups.iter().all(|g| matches!(g.team, Team::Infection)) {
        Some(Team::Infection)
    } else {
        None
    }
}
