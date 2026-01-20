use crate::parsing::parse_input;
use fxhash::FxHashMap;

pub fn process<'a>(_input: &'a str) -> usize {
    let input = parse_input(_input);

    let mut map: FxHashMap<&'a str, Vec<&'a str>> = FxHashMap::default();

    for (a, b) in input {
        map.entry(a).and_modify(|n| n.push(b)).or_insert(vec![b]);
        map.entry(b).and_modify(|n| n.push(a)).or_insert(vec![a]);
    }

    let mut ans = 1;

    let mut current: Vec<(&str, &str)> = vec![(
        "YOU",
        map.remove("YOU").unwrap().into_iter().next().unwrap(),
    )];

    'a: loop {
        let mut new: Vec<(&str, &str)> = vec![];

        ans += 1;
        for (a, b) in current {
            for c in map.remove(b).unwrap().into_iter().filter(|p| p != &a) {
                if c == "SAN" {
                    break 'a;
                }
                new.push((b, c));
            }
        }

        current = new;
    }

    ans - 2
}
