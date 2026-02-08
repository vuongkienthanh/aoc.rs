pub mod parsing;
pub mod part1;
pub mod part2;

#[derive(Debug, Clone, Copy)]
struct Character {
    hp: usize,
    atk: usize,
    def: usize,
}

#[derive(Clone, Copy)]
struct Weapon {
    cost: usize,
    atk: usize,
}
#[derive(Clone, Copy)]
struct Armor {
    cost: usize,
    def: usize,
}
#[derive(Clone, Copy)]
struct Ring {
    cost: usize,
    atk: usize,
    def: usize,
}
const WEAPONS: [Weapon; 5] = [
    Weapon { cost: 8, atk: 4 },
    Weapon { cost: 10, atk: 5 },
    Weapon { cost: 25, atk: 6 },
    Weapon { cost: 40, atk: 7 },
    Weapon { cost: 75, atk: 8 },
];
const ARMORS: [Armor; 6] = [
    Armor { cost: 0, def: 0 },
    Armor { cost: 13, def: 1 },
    Armor { cost: 31, def: 2 },
    Armor { cost: 53, def: 3 },
    Armor { cost: 75, def: 4 },
    Armor { cost: 102, def: 5 },
];

const RINGS: [Ring; 7] = [
    Ring {
        cost: 0,
        atk: 0,
        def: 0,
    },
    Ring {
        cost: 25,
        atk: 1,
        def: 0,
    },
    Ring {
        cost: 50,
        atk: 2,
        def: 0,
    },
    Ring {
        cost: 100,
        atk: 3,
        def: 0,
    },
    Ring {
        cost: 20,
        atk: 0,
        def: 1,
    },
    Ring {
        cost: 40,
        atk: 0,
        def: 2,
    },
    Ring {
        cost: 80,
        atk: 0,
        def: 3,
    },
];

fn ring_combinations() -> Vec<(Ring, Ring)> {
    let mut ret = vec![(
        Ring {
            cost: 0,
            atk: 0,
            def: 0,
        },
        Ring {
            cost: 0,
            atk: 0,
            def: 0,
        },
    )];
    for (i, l) in RINGS.iter().enumerate() {
        for r in &RINGS[i + 1..] {
            ret.push((*l, *r));
        }
    }
    ret
}

impl Character {
    fn boss(hp: usize, atk: usize, def: usize) -> Self {
        Self { hp, atk, def }
    }
    fn hero(w: Weapon, a: Armor, r: (Ring, Ring)) -> Self {
        Self {
            hp: 100,
            atk: w.atk + r.0.atk + r.1.atk,
            def: a.def + r.0.def + r.1.def,
        }
    }
    fn attack(&self, other: &Self) -> usize {
        self.atk.checked_sub(other.def).unwrap_or(1).max(1)
    }
}

fn equipments_cost(w: Weapon, a: Armor, r: (Ring, Ring)) -> usize {
    w.cost + a.cost + r.0.cost + r.1.cost
}
fn battle(mut hero: Character, mut boss: Character) -> bool {
    loop {
        // hero atk first
        let damage = hero.attack(&boss);
        if boss.hp <= damage {
            return true;
        } else {
            boss.hp -= damage;
        }
        // boss atk second
        let damage = boss.attack(&hero);
        if hero.hp <= damage {
            return false;
        } else {
            hero.hp -= damage;
        }
    }
}
