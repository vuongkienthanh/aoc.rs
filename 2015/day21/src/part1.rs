use crate::parsing::parse_input;
use crate::{ARMORS, Character, WEAPONS, battle, equipments_cost, ring_combinations};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let boss = Character::boss(input.0, input.1, input.2);

    let mut ans = usize::MAX;
    for w in WEAPONS {
        for a in ARMORS {
            for r in ring_combinations() {
                let hero = Character::hero(w, a, r);
                let cost = equipments_cost(w, a, r);
                if battle(hero, boss) {
                    ans = ans.min(cost);
                }
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_battle() {
        let hero = Character {
            hp: 8,
            atk: 5,
            def: 5,
        };
        let boss = Character {
            hp: 12,
            atk: 7,
            def: 2,
        };
        assert!(battle(hero, boss));
    }
}
