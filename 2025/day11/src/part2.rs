use crate::create_map;
use crate::parsing::parse_input;
use fxhash::FxHashMap;

type Cache<'a> = FxHashMap<(&'a str, bool, bool), usize>;
type Map<'a> = FxHashMap<&'a str, Vec<&'a str>>;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let input = create_map(input);

    let mut cache = Cache::default();

    solve(("svr", false, false), &input, &mut cache)
}
fn solve<'a>(node: (&'a str, bool, bool), input: &Map<'a>, cache: &mut Cache<'a>) -> usize {
    if node.0 == "out" {
        if node.1 && node.2 {
            return 1;
        } else {
            return 0;
        }
    }
    let mut ans = 0;
    for dst in input.get(&node.0).unwrap() {
        let dac = node.1 || *dst == "dac";
        let fft = node.2 || *dst == "fft";
        let x = cache
            .get(&(dst, dac, fft))
            .cloned()
            .unwrap_or_else(|| solve((dst, dac, fft), input, cache));
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
        r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 2);
    }
}
