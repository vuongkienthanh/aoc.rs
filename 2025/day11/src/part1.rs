use crate::create_map;
use crate::parsing::parse_input;
use fxhash::FxHashMap;

type Cache<'a> = FxHashMap<&'a str, usize>;
type Map<'a> = FxHashMap<&'a str, Vec<&'a str>>;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let input = create_map(input);
    // println!("{input:?}");

    let mut cache = Cache::default();

    solve("you", &input, &mut cache)
}

fn solve<'a>(node: &'a str, input: &Map<'a>, cache: &mut Cache<'a>) -> usize {
    if node == "out" {
        return 1;
    }
    // all other nodes should have out cables
    let mut ans = 0;
    for dst in input.get(&node).unwrap() {
        let x = cache
            .get(dst)
            .cloned()
            .unwrap_or_else(|| solve(dst, input, cache));
        ans += x;
    }
    cache.insert(node, ans);
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 5);
    }
}
