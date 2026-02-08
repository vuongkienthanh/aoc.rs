use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::{many1, separated_list1},
};

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum Item {
    #[default]
    Space,
    Wall,
    Goblin(usize),
    Elf(usize),
}

impl Item {
    pub fn is_enemy_of(&self, other: &Item) -> bool {
        use Item::*;
        matches!((self, other), (Goblin(_), Elf(_)) | (Elf(_), Goblin(_)))
    }
    /// return true if dead
    pub fn got_hit_and_is_dead(&mut self, atk_power: usize) -> bool {
        use Item::*;
        let mut is_dead = false;
        match self {
            Space | Wall => panic!("should be a unit"),
            Goblin(x) => match x.checked_sub(atk_power) {
                None | Some(0) => {
                    *self = Item::Space;
                    is_dead = true;
                }
                Some(y) => *self = Item::Goblin(y),
            },
            Elf(x) => match x.checked_sub(atk_power) {
                None | Some(0) => {
                    *self = Item::Space;
                    is_dead = true;
                }
                Some(y) => *self = Item::Elf(y),
            },
        }
        is_dead
    }
    pub fn is_elf(&self) -> bool {
        matches!(self, Item::Elf(_))
    }
    pub fn is_goblin(&self) -> bool {
        matches!(self, Item::Goblin(_))
    }
    pub fn hp(&self) -> usize {
        use Item::*;
        match self {
            Space | Wall => 0,
            Goblin(x) | Elf(x) => *x,
        }
    }
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    alt((
        complete::char('#').map(|_| Item::Wall),
        complete::char('.').map(|_| Item::Space),
        complete::char('G').map(|_| Item::Goblin(200)),
        complete::char('E').map(|_| Item::Elf(200)),
    ))
    .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Item>> {
    many1(parse_item).parse(input)
}

pub fn parse_input(input: &str) -> Vec<Vec<Item>> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
