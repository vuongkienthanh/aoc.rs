pub mod parsing;
pub mod part1;
pub mod part2;

struct Boss {
    hp: usize,
    atk: usize,
    def: usize,
}

struct Weapon {
    cost: usize,
    atk: usize,
}
struct NoWeapon;
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
struct Character<W> {
    hp: usize,
    weapon: W,
    armor: Option<Armor>,
    rings: [Option<Ring>; 2],
}

impl Character<NoWeapon> {
    fn new(hp: usize) -> Self {
        Self {
            hp,
            weapon: NoWeapon,
            armor: None,
            rings: [None; 2],
        }
    }
    fn add_weapon(self, weapon: Weapon) -> Character<Weapon> {
        Character {
            hp: self.hp,
            weapon,
            armor: self.armor,
            rings: self.rings,
        }
    }
}
impl<W> Character<W> {
    fn replace_rings(mut self, rings: [Option<Ring>; 2]) -> Character<W> {
        self.rings = rings;
        self
    }
    fn replace_armor(mut self, armor: Option<Armor>) -> Character<W> {
        self.armor = armor;
        self
    }
}

trait Defend {
    fn def(&self) -> usize;
}
impl Defend for Boss {
    fn def(&self) -> usize {
        self.def
    }
}
impl<W> Defend for Character<W> {
    fn def(&self) -> usize {
        let mut ans = 0;
        if let Some(Armor { def, .. }) = self.armor {
            ans += def
        }
        if let Some(Ring { def, .. }) = self.rings[0] {
            ans += def
        }
        if let Some(Ring { def, .. }) = self.rings[1] {
            ans += def
        }
        ans
    }
}

trait Attackable {
    fn atk(&self) -> usize;
    fn attack(&self, def: usize) -> usize {
        self.atk().checked_sub(def).unwrap_or(1).max(1)
    }
}
impl Attackable for Boss {
    fn atk(&self) -> usize {
        self.atk
    }
}
impl Attackable for Character<Weapon> {
    fn atk(&self) -> usize {
        let mut ans = 0;
        let Weapon { atk, .. } = self.weapon;
        ans += atk;
        if let Some(Ring { atk, .. }) = self.rings[0] {
            ans += atk
        }
        if let Some(Ring { atk, .. }) = self.rings[1] {
            ans += atk
        }
        ans
    }
}
