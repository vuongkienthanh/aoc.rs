use crate::Resource;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut ans = 0;

    for bp in input {
        print!(
            "{} {}: ",
            bp.last_chance_build_clay, bp.last_chance_build_obsidian
        );
        let mut current = vec![(Resource::new(), 0)];
        for _ in 0..=bp.last_chance_build_clay {
            let mut new = vec![];
            for (mut resource, mut flag) in current {
                if bp.can_make_ore_robot(&resource) {
                    if flag & 1 == 0 {
                        let resource = bp.make_ore_robot(&resource);
                        new.push((resource, 0));
                    }
                    flag |= 1;
                }
                if bp.can_make_clay_robot(&resource) {
                    if flag & 0b10 == 0 {
                        let resource = bp.make_clay_robot(&resource);
                        new.push((resource, 0));
                    }
                    flag |= 0b10;
                }
                if bp.can_make_obsidian_robot(&resource) {
                    if flag & 0b100 == 0 {
                        let resource = bp.make_obsidian_robot(&resource);
                        new.push((resource, 0));
                    }
                    flag |= 0b100;
                }
                if (resource.clay == 0 && flag == 0b11) || (resource.obsidian == 0 && flag == 0b111)
                {
                    continue;
                } else {
                    resource.step();
                    new.push((resource, flag));
                }
            }
            current = new;
        }
        print!("{}->", current.len());
        current.retain(|(r, _)| r.clay_robot > 0);
        print!("{}; ", current.len());
        for _ in bp.last_chance_build_clay + 1..=bp.last_chance_build_obsidian {
            let mut new = vec![];
            for (mut resource, mut flag) in current {
                if bp.can_make_geode_robot(&resource) {
                    let resource = bp.make_geode_robot(&resource);
                    new.push((resource, 0));
                    continue;
                }
                if bp.can_make_ore_robot(&resource) {
                    if flag & 1 == 0 {
                        let resource = bp.make_ore_robot(&resource);
                        new.push((resource, 0));
                    }
                    flag |= 1;
                }
                if bp.can_make_clay_robot(&resource) {
                    if flag & 0b10 == 0 {
                        let resource = bp.make_clay_robot(&resource);
                        new.push((resource, 0));
                    }
                    flag |= 0b10;
                }
                if bp.can_make_obsidian_robot(&resource) {
                    if flag & 0b100 == 0 {
                        let resource = bp.make_obsidian_robot(&resource);
                        new.push((resource, 0));
                    }
                    flag |= 0b100;
                }
                resource.step();
                new.push((resource, flag));
            }
            current = new;
        }
        print!("{}->", current.len());
        current.retain(|(r, _)| r.obsidian_robot > 0);
        print!("{}; ", current.len());
        for _ in bp.last_chance_build_obsidian + 1..22 {
            let mut new = vec![];
            for (mut resource, mut flag) in current {
                if bp.can_make_geode_robot(&resource) {
                    let resource = bp.make_geode_robot(&resource);
                    new.push((resource, 0));
                    continue;
                }
                if bp.can_make_ore_robot(&resource) {
                    if flag & 1 == 0 {
                        let resource = bp.make_ore_robot(&resource);
                        new.push((resource, 0));
                    }
                    flag |= 1;
                }
                if bp.can_make_clay_robot(&resource) {
                    if flag & 0b10 == 0 {
                        let resource = bp.make_clay_robot(&resource);
                        new.push((resource, 0));
                    } else {
                        flag |= 0b10;
                    }
                }
                if bp.can_make_obsidian_robot(&resource) {
                    if flag & 0b100 == 0 {
                        let resource = bp.make_obsidian_robot(&resource);
                        new.push((resource, 0));
                    } else {
                        flag |= 0b100;
                    }
                }
                resource.step();
                new.push((resource, flag));
            }
            current = new;
        }
        println!("{}", current.len());
        ans += bp.id
            * current
                .into_iter()
                .map(|(r, _)| {
                    if bp.can_make_geode_robot(&r) {
                        r.geode + r.geode_robot * 2 + 1
                    } else {
                        r.geode + r.geode_robot * 2
                    }
                })
                .max()
                .unwrap_or_default();
    }

    ans
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
