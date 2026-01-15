pub mod part1;
pub mod part2;
pub mod parsing;

use std::collections::BTreeMap;
use parsing::{Dir, Item};

type Point = (isize, isize);
pub type Map = BTreeMap<Point, usize>;
type Actor = (Point, usize);

pub fn run_seq(actor: Actor, map: &mut Map, items: &[Item]) -> Vec<Actor> {
    let mut actors = vec![actor];
    for item in items {
        let mut new_actors = vec![];

        for actor in actors {
            new_actors.extend(run(actor, map, item));
        }
        actors = new_actors;
    }
    actors.sort_unstable();
    actors.dedup();
    actors
}

fn run(mut actor: Actor, map: &mut Map, item: &Item) -> Vec<Actor> {
    let mut actors = vec![];
    match item {
        Item::Dirs(v) => {
            for d in v {
                let mut new_actor = match d {
                    Dir::E => ((actor.0.0, actor.0.1 + 1), actor.1 + 1),
                    Dir::W => ((actor.0.0, actor.0.1 - 1), actor.1 + 1),
                    Dir::N => ((actor.0.0 - 1, actor.0.1), actor.1 + 1),
                    Dir::S => ((actor.0.0 + 1, actor.0.1), actor.1 + 1),
                };
                new_actor.1 = map
                    .entry(new_actor.0)
                    .and_modify(|x| *x = (*x).min(new_actor.1))
                    .or_insert(new_actor.1).clone();
                actor = new_actor;
            }
            actors.push(actor);
        }
        Item::Branch(bs) => {
            for b in bs {
                actors.extend(run_seq(actor.clone(), map, b));
            }
        }
    };
    actors
}
