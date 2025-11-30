use crate::parsing::parse_input;
use crate::walk;
use itertools::Itertools;
use std::collections::BTreeMap;

pub fn process(_input: &str) -> usize {
    let (g, number_locations) = parse_input(_input);

    let mut pairwise: BTreeMap<char, BTreeMap<char, usize>> = BTreeMap::new();

    for from in number_locations.iter().map(|(k, v)| (*k, *v)) {
        pairwise.insert(from.0, walk(from.1, &g, number_locations.len()));
    }

    let mut min = usize::MAX;
    let perm_len = number_locations.len() - 1;
    for v in number_locations
        .into_keys()
        .filter(|x| *x != '0')
        .permutations(perm_len)
    {
        let mut sum = pairwise.get(&'0').unwrap().get(&v[0]).cloned().unwrap();
        for v2 in v.windows(2) {
            sum += pairwise.get(&v2[0]).unwrap().get(&v2[1]).cloned().unwrap();
        }
        sum += pairwise
            .get(v.last().unwrap())
            .unwrap()
            .get(&'0')
            .cloned()
            .unwrap();

        min = min.min(sum);
    }

    min
}
