use crate::parsing::parse_input;
use crate::{ARMORS, Character, WEAPONS, battle, equipments_cost, ring_combinations};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let boss = Character::boss(input.0, input.1, input.2);

    let mut ans = 0;
    for w in WEAPONS {
        for a in ARMORS {
            for r in ring_combinations() {
                let hero = Character::hero(w, a, r);
                let cost = equipments_cost(w, a, r);
                if !battle(hero, boss) {
                    ans = ans.max(cost);
                }
            }
        }
    }
    ans
}
