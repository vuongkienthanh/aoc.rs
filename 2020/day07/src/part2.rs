use crate::parsing::parse_input;
use fxhash::FxHashMap;

type Map<'a> = FxHashMap<&'a str, Vec<(usize, &'a str)>>;
type Cache<'a> = FxHashMap<&'a str, usize>;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let map = Map::from_iter(input.into_iter());
    let mut cache = Cache::default();
    can_hold("shiny gold", &map, &mut cache)
}
fn can_hold<'a>(x: &'a str, map: &Map<'a>, cache: &mut Cache<'a>) -> usize {
    if let Some(ans) = cache.get(x).cloned() {
        ans
    } else {
        let v = map.get(x).expect("found bag");
        let ans = v.iter().map(|(i, b)| i + i * can_hold(b, map, cache)).sum();
        cache.insert(x, ans);
        ans
    }
}
