use crate::parsing::parse_input;
use crate::{Cache, Counter, add_counter, polymerization};

pub fn process(_input: &str) -> usize {
    let (template, map) = parse_input(_input);
    let mut cache = Cache::default();
    let mut ans = Counter::default();
    for w in template.windows(2) {
        let (a, b) = (w[0], w[1]);
        let mid = polymerization((a, b, 40), &map, &mut cache);
        *ans.entry(a).or_default() += 1;
        add_counter(&mut ans, mid);
    }
    *ans.entry(template.last().cloned().unwrap()).or_default() += 1;
    let max = ans.values().max().unwrap();
    let min = ans.values().min().unwrap();
    max - min
}
