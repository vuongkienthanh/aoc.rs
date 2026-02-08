use crate::parsing::parse_input;
use fxhash::FxHashMap;

type Map<'a> = FxHashMap<&'a str, Vec<(usize, &'a str)>>;
type Cache<'a> = FxHashMap<&'a str, bool>;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let map = Map::from_iter(input.into_iter());
    let mut cache = Cache::default();
    map.iter()
        .filter(|(x, _)| can_contain_shiny_gold(*x, &map, &mut cache))
        .count()
}

fn can_contain_shiny_gold<'a>(x: &'a str, map: &Map<'a>, cache: &mut Cache<'a>) -> bool {
    if let Some(ans) = cache.get(x).cloned() {
        ans
    } else {
        let v = map.get(x).expect("found bag");
        let ans = v
            .iter()
            .any(|(_, b)| *b == "shiny gold" || can_contain_shiny_gold(*b, map, cache));
        cache.insert(x, ans);
        ans
    }
}
