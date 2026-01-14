pub mod parsing;
pub mod part1;
pub mod part2;
use std::collections::BTreeMap;
pub type Point = (usize, usize);
pub type Cache = BTreeMap<Point, usize>;

fn get_geo(p: Point, depth: usize, target: Point, cache: &mut Cache) -> usize {
    match p {
        (0, 0) => 0,
        loc if loc == target => 0,
        (x, 0) => x * 16807,
        (0, y) => y * 48271,
        _ => {
            get_ero((p.0 - 1, p.1), depth, target, cache)
                * get_ero((p.0, p.1 - 1), depth, target, cache)
        }
    }
}

fn get_ero(p: Point, depth: usize, target: Point, cache: &mut Cache) -> usize {
    if let Some(x) = cache.get(&p) {
        return *x;
    }
    let ero = (get_geo(p, depth, target, cache) + depth) % 20183;
    cache.insert(p, ero);
    ero
}

fn get_type(p: Point, depth: usize, target: Point, cache: &mut Cache) -> usize {
    get_ero(p, depth, target, cache) % 3
}
