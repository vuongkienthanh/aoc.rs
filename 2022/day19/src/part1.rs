use crate::Resource;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut ans = 0;

    for bp in input {
        let mut current = vec![(
            Resource {
                ore_robot: 1,
                clay_robot: 0,
                obsidian_robot: 0,
                geode_robot: 0,
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            0,
        )];
        for i in 0..12 {
            let mut new = vec![];
            for (resource, flag) in current {
                // if not skip ore robot
                if flag & 1 == 0 {
                    // make ore robot
                    if bp.can_make_ore_robot(&resource) {
                        new.push((bp.make_ore_robot(&resource), 0));
                    }
                    // or just skip ore robot
                    let mut resource = resource.clone();
                    resource.step();
                    new.push((resource, 0b1));
                }
                // if not skip clay robot
                if flag & 0b10 == 0 {
                    // make clay robot
                    if bp.can_make_clay_robot(&resource) {
                        new.push((bp.make_clay_robot(&resource), 0));
                    }
                    // or just skip clay robot
                    let mut resource = resource.clone();
                    resource.step();
                    new.push((resource, flag | 0b10));
                }
                if resource.clay_robot > 0 {
                    // if not skip obsidian robot
                    if flag & 0b100 == 0 {
                        // make obsidian robot
                        if bp.can_make_obsidian_robot(&resource) {
                            new.push((bp.make_obsidian_robot(&resource), 0));
                        }
                        // or just skip obsidian robot
                        let mut resource = resource.clone();
                        resource.step();
                        new.push((resource, flag | 0b100));
                    }
                }
                if resource.obsidian_robot > 0 {
                    // make geode if possible
                    if bp.can_make_geode_robot(&resource) {
                        new.push((bp.make_geode_robot(&resource), 0));
                    }
                    // or just step
                    let mut resource = resource.clone();
                    resource.step();
                    new.push((resource, flag));
                }
            }
            current = new;
            if i == 8 {
                current.retain(|(r, _)| r.clay_robot > 0);
            }
            // if i == 12 { current.retain(|(r, _)| r.obsidian_robot > 0);
            // }
            println!("{}", current.len());
        }

        ans += bp.id * dbg!(current.into_iter().map(|(r, _)| r.geode).max().unwrap());
        if true {
            panic!();
        }
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
