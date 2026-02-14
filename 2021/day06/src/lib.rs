pub mod part1;
pub mod part2;
use fxhash::FxHashMap;

pub type Cache = FxHashMap<(usize, usize), usize>;

pub fn get_cache() -> Cache {
    let mut cache = Cache::default();
    // base cases
    cache.insert((6, 9), 2);
    cache.insert((6, 8), 2);
    cache.insert((6, 7), 2);
    cache.insert((6, 6), 1);
    cache.insert((6, 5), 1);
    cache.insert((6, 4), 1);
    cache.insert((6, 3), 1);
    cache.insert((6, 2), 1);
    cache.insert((6, 1), 1);
    cache
}

pub fn count(fish: usize, day: usize, cache: &mut Cache) -> usize {
    if let Some(ans) = cache.get(&(fish, day)).copied() {
        return ans;
    } else {
        let ans = count(6, day - fish - 1, cache) + count(6, day - fish - 3, cache);
        cache.insert((fish, day), ans);
        ans
    }
}
