pub mod parsing;
pub mod part1;
pub mod part2;

use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
struct Boss {
    hp: usize,
    atk: usize,
}
#[derive(Debug, Clone, Copy)]
struct Hero {
    hp: usize,
    mana: usize,
    used_mana: usize,
    def: usize,
}
#[derive(Debug, Clone, Copy)]
enum Spell {
    MagicMissile(usize),
    Drain(usize),
    Shield(usize),
    Poison(usize),
    Recharge(usize),
}
const SPELLS: [Spell; 5] = [
    Spell::MagicMissile(53),
    Spell::Drain(73),
    Spell::Shield(113),
    Spell::Poison(173),
    Spell::Recharge(229),
];

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Eq, PartialEq, Debug)]
enum IsHeroWin {
    YES,
    NO,
    NOT_YET,
}

#[derive(Debug, Clone)]
struct Battle {
    hero: Hero,
    boss: Boss,

    in_effect_shield: usize,
    in_effect_poison: usize,
    in_effect_recharge: usize,
}
impl Battle {
    pub fn new(hero: Hero, boss: Boss) -> Self {
        Self {
            hero,
            boss,
            in_effect_shield: 0,
            in_effect_poison: 0,
            in_effect_recharge: 0,
        }
    }
    fn deal_damage_boss(&mut self, damage: usize) -> IsHeroWin {
        if damage >= self.boss.hp {
            IsHeroWin::YES
        } else {
            self.boss.hp -= damage;
            IsHeroWin::NOT_YET
        }
    }
    fn deal_damage_hero(&mut self, damage: usize) -> IsHeroWin {
        let damage = damage.checked_sub(self.hero.def).unwrap_or(1).max(1);
        if damage >= self.hero.hp {
            IsHeroWin::NO
        } else {
            self.hero.hp -= damage;
            IsHeroWin::NOT_YET
        }
    }
    fn solve_effect_shield(&mut self) -> IsHeroWin {
        if self.in_effect_shield > 0 {
            self.in_effect_shield -= 1;
            if self.in_effect_shield == 0 {
                self.hero.def = 0;
            }
        }
        IsHeroWin::NOT_YET
    }
    fn solve_effect_poison(&mut self) -> IsHeroWin {
        if self.in_effect_poison > 0 {
            self.in_effect_poison -= 1;
            return self.deal_damage_boss(3);
        }
        IsHeroWin::NOT_YET
    }
    fn solve_effect_recharge(&mut self) -> IsHeroWin {
        if self.in_effect_recharge > 0 {
            self.in_effect_recharge -= 1;
            self.hero.mana += 101;
        }
        IsHeroWin::NOT_YET
    }
    fn solve_effect(&mut self) -> IsHeroWin {
        if [
            Self::solve_effect_shield,
            Self::solve_effect_poison,
            Self::solve_effect_recharge,
        ]
        .into_iter()
        .any(|f| f(self) == IsHeroWin::YES)
        {
            return IsHeroWin::YES;
        }
        IsHeroWin::NOT_YET
    }
    fn hero_turn(&mut self, use_spell: Spell) -> IsHeroWin {
        if self.solve_effect() == IsHeroWin::YES {
            return IsHeroWin::YES;
        }
        match use_spell {
            Spell::MagicMissile(cost) => {
                if self.hero.mana >= cost {
                    self.hero.mana -= cost;
                    self.hero.used_mana += cost;
                    self.deal_damage_boss(4)
                } else {
                    IsHeroWin::NO
                }
            }
            Spell::Drain(cost) => {
                if self.hero.mana >= cost {
                    self.hero.mana -= cost;
                    self.hero.used_mana += cost;
                    self.hero.hp += 2;
                    self.deal_damage_boss(2)
                } else {
                    IsHeroWin::NO
                }
            }
            Spell::Shield(cost) => {
                if self.hero.mana >= cost {
                    self.hero.mana -= cost;
                    self.hero.used_mana += cost;
                    self.hero.def = 7;
                    self.in_effect_shield = 6;
                    IsHeroWin::NOT_YET
                } else {
                    IsHeroWin::NO
                }
            }
            Spell::Poison(cost) => {
                if self.hero.mana >= cost {
                    self.hero.mana -= cost;
                    self.hero.used_mana += cost;
                    self.in_effect_poison = 6;
                    IsHeroWin::NOT_YET
                } else {
                    IsHeroWin::NO
                }
            }
            Spell::Recharge(cost) => {
                if self.hero.mana >= cost {
                    self.hero.mana -= cost;
                    self.hero.used_mana += cost;
                    self.in_effect_recharge = 5;
                    IsHeroWin::NOT_YET
                } else {
                    IsHeroWin::NO
                }
            }
        }
    }
    fn boss_turn(&mut self) -> IsHeroWin {
        if self.solve_effect() == IsHeroWin::YES {
            return IsHeroWin::YES;
        }
        self.deal_damage_hero(self.boss.atk)
    }
    fn run(&mut self, spell: Spell) -> IsHeroWin {
        match self.hero_turn(spell) {
            IsHeroWin::NOT_YET => self.boss_turn(),
            x => x,
        }
    }
    fn hero_turn2(&mut self, use_spell: Spell) -> IsHeroWin {
        self.hero.hp -= 1;
        if self.hero.hp == 0 {
            return IsHeroWin::NO;
        }
        self.hero_turn(use_spell)
    }
    fn run2(&mut self, spell: Spell) -> IsHeroWin {
        match self.hero_turn2(spell) {
            IsHeroWin::NOT_YET => self.boss_turn(),
            x => x,
        }
    }
}

impl Ord for Battle {
    fn cmp(&self, other: &Self) -> Ordering {
        other.hero.used_mana.cmp(&self.hero.used_mana)
    }
}
impl PartialOrd for Battle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Battle {}
impl PartialEq for Battle {
    fn eq(&self, other: &Self) -> bool {
        self.hero.used_mana == other.hero.used_mana
    }
}
