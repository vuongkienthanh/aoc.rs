use crate::parsing::{Item, parse_input};
use fxhash::FxHashMap;

type Room = FxHashMap<(isize, isize), bool>;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    input
        .into_iter()
        .fold(Room::default(), |mut acc, line| {
            let (mut ns, mut ew) = (0, 0);
            for c in line {
                match c {
                    Item::E => ew += 1,
                    Item::W => ew -= 1,
                    Item::NW => ns += 1,
                    Item::SE => ns -= 1,
                    Item::NE => {
                        ns += 1;
                        ew += 1;
                    }
                    Item::SW => {
                        ns -= 1;
                        ew -= 1;
                    }
                }
            }
            acc.entry((ns, ew))
                .and_modify(|x| *x = !(*x))
                .or_insert(true);
            acc
        })
        .into_values()
        .filter(|x| *x)
        .count()
}
