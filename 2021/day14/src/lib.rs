pub mod parsing;
pub mod part1;
pub mod part2;

use fxhash::FxHashMap;
use parsing::Map;
pub type Counter = FxHashMap<char, usize>;
pub type Cache = FxHashMap<(char, char, usize), Counter>;

fn polymerization(
    (a, b, i): (char, char, usize),
    map: &Map,
    cache: &mut Cache,
) -> FxHashMap<char, usize> {
    if let Some(mid) = cache.get(&(a, b, i)).cloned() {
        mid
    } else {
        let mut ans = Counter::default();
        let m: char = map.get(&(a, b)).cloned().unwrap();
        *ans.entry(m).or_default() += 1;
        if i > 1 {
            add_counter(&mut ans, polymerization((a, m, i - 1), map, cache));
            add_counter(&mut ans, polymerization((m, b, i - 1), map, cache));
        }
        cache.insert((a, b, i), ans.clone());
        ans
    }
}

fn add_counter(a: &mut Counter, b: Counter) {
    for (k, v) in b {
        *a.entry(k).or_default() += v;
    }
}
