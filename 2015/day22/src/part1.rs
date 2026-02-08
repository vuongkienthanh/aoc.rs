use crate::parsing::parse_input;
use crate::{Battle, Boss, Hero, IsHeroWin, SPELLS};
use std::collections::BinaryHeap;
pub fn process(_input: &str) -> usize {
    let (hp, atk) = parse_input(_input);
    find_least_mana(hp, atk)
}
fn find_least_mana(hp: usize, atk: usize) -> usize {
    let mut heap = BinaryHeap::new();
    heap.push(Battle::new(
        Hero {
            hp: 50,
            mana: 500,
            used_mana: 0,
            def: 0,
        },
        Boss { hp, atk },
    ));

    while let Some(battle) = heap.pop() {
        for spell in SPELLS {
            let mut new_battle = battle.clone();
            let ret = new_battle.run(spell);
            match ret {
                IsHeroWin::YES => {
                    return new_battle.hero.used_mana;
                }
                IsHeroWin::NO => (),
                IsHeroWin::NOT_YET => heap.push(new_battle),
            }
        }
    }

    panic!("should have an answer")
}
