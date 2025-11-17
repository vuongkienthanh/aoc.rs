pub mod parsing;
pub mod part1;
pub mod part2;

#[derive(Debug, Clone, Copy)]
struct Boss {
    hp: usize,
    atk: usize,
}
struct Hero {
    hp: usize,
    mana: usize,
    used_mana: usize,
    def: usize,
}
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

struct Battle {
    hero: Hero,
    boss: Boss,

    in_effect_drain: usize,
    in_effect_shield: usize,
    in_effect_poison: usize,
    in_effect_recharge: usize,
}
impl Battle {
    fn deal_damage_boss(&mut self, damage: usize) -> bool {
        if damage >= self.boss.hp {
            return true;
        } else {
            self.boss.hp -= damage;
            return false;
        }
    }
    fn deal_damage_hero(&mut self, damage: usize) -> bool {
        let damage = damage.checked_sub(self.hero.def).unwrap_or(1).max(1);
        if damage >= self.hero.hp {
            return true;
        } else {
            self.hero.hp -= damage;
            return false;
        }
    }
    fn solve_drain_effect(&mut self) -> bool {
        if self.in_effect_drain > 0 {
            self.in_effect_drain -= 1;
            if self.deal_damage_boss(2) {
                return true;
            } else {
                self.hero.hp += 2
            }
        }
        false
    }
    fn solve_shield_effect(&mut self) -> bool {
        if self.in_effect_shield > 0 {
            self.in_effect_shield -= 1;
            self.hero.def = 0;
        }
        false
    }
    fn solve_poison_effect(&mut self) -> bool {
        if self.in_effect_poison > 0 {
            self.in_effect_poison -= 1;
            if self.deal_damage_boss(3) {
                return true;
            }
        }
        false
    }
    fn solve_recharge_effect(&mut self) -> bool {
        if self.in_effect_recharge > 0 {
            self.in_effect_recharge -= 1;
            self.hero.mana += 101
        }
        false
    }
    fn hero_turn(&mut self, use_spell: Spell) -> bool {
        if [
            Self::solve_drain_effect,
            Self::solve_shield_effect,
            Self::solve_poison_effect,
            Self::solve_recharge_effect,
        ]
        .into_iter()
        .any(|f| f(self))
        {
            return true;
        }
        match use_spell {
            Spell::MagicMissile(cost) => {
                if self.hero.mana >= cost {
                    self.hero.mana -= cost;
                    self.hero.used_mana += cost;
                    return self.deal_damage_boss(4);
                }
                false
            }
            Spell::Drain(cost) => {
                if self.hero.mana > cost {
                    self.hero.mana -= cost;
                    self.hero.used_mana += cost;
                    self.hero.hp += 2;
                    return self.deal_damage_boss(2);
                }
                false
            }
            Spell::Shield(cost) => {
                if self.hero.mana > cost {
                    self.hero.mana -= cost;
                    self.hero.used_mana += cost;
                    self.hero.def = 7;
                    self.in_effect_shield = 6;
                }
                false
            }
            Spell::Poison(cost) => {
                if self.hero.mana > cost {
                    self.hero.mana -= cost;
                    self.hero.used_mana += cost;
                    self.in_effect_poison = 6;
                }
                false
            }
            Spell::Recharge(cost) => {
                if self.hero.mana > cost {
                    self.hero.mana -= cost;
                    self.hero.used_mana += cost;
                    self.in_effect_recharge = 5;
                }
                false
            }
        }
    }
    fn boss_turn(&mut self) -> bool {
        self.deal_damage_hero(self.boss.atk)
    }
}
