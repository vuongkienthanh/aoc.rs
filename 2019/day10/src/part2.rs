use crate::parsing::parse;
use crate::part1::find_best_location;
use fraction::GenericFraction;
use std::collections::BTreeMap;
type F = GenericFraction<usize>;
type Map = BTreeMap<(usize, F), Vec<(usize, usize)>>;

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    let (_, best) = find_best_location(&input);

    let mut map = Map::new();
    for p in input {
        if p == best {
            continue;
        }
        let deg = degree(p, best);
        map.entry(deg).or_default().push(p);
    }

    for v in map.values_mut() {
        v.sort_unstable_by(|a, b| manhattan(*b, best).cmp(&manhattan(*a, best)));
    }

    let mut step = 0;
    'a: loop {
        for v in map.values_mut() {
            step += 1;
            let p = v.pop().unwrap();
            if step == 200 {
                break 'a p.0 * 100 + p.1;
            }
        }
    }
}
fn manhattan(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

// return quarter and fraction
fn degree((x, y): (usize, usize), best: (usize, usize)) -> (usize, F) {
    let dx = x.abs_diff(best.0);
    let dy = y.abs_diff(best.1);
    match (x >= best.0, y >= best.1) {
        (false, true) => (0, F::new(dx, dy)),
        (true, true) => (1, F::new(dy, dx)),
        (true, false) => (2, F::new(dx, dy)),
        (false, false) => (3, F::new(dy, dx)),
    }
}
