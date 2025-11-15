use crate::parsing::{Map, parse_input, parse_molecules};
use std::collections::HashMap;

pub fn process(_input: &str) -> usize {
    let (_, (map, molecules)) = parse_input(_input).unwrap();
    find_min(map, molecules)
}

type Cache<'a> = HashMap<Vec<&'a str>, HashMap<&'a str, usize>>;
fn find_min<'a>(map: Map<'a>, molecules: Vec<&'a str>) -> usize {
    let mut cache = Cache::new();

    for (k, v) in map {
        for i in v {
            let (_, expanded) = parse_molecules(i).unwrap();
            cache.entry(expanded).or_default().insert(k, 1);
        }
        cache.entry(vec![k]).or_default().insert(k, 0);
    }

    build(molecules.clone(), &mut cache);

    if let Some(ans) = cache.get(&molecules).unwrap().get("e") {
        *ans
    } else {
        let mut min = usize::MAX;
        for (k, v) in cache.get(&molecules).unwrap() {
            let m = cache
                .get(&vec![*k])
                .cloned()
                .unwrap_or_default()
                .get("e")
                .cloned()
                .unwrap_or(usize::MAX);
            min = min.min(m + v);
        }
        min
    }
}

fn build<'a>(molecules: Vec<&'a str>, cache: &mut Cache<'a>) {
    if molecules.len() == 1 || molecules.len() == 2 {
        return;
    }
    for i in 1..molecules.len() {
        let left_molecules = &molecules[..i];
        let right_molecules = &molecules[i..];
        build(left_molecules.to_vec(), cache);
        build(right_molecules.to_vec(), cache);

        let left_hm = cache.get(left_molecules).cloned().unwrap_or_default();
        let right_hm = cache.get(right_molecules).cloned().unwrap_or_default();

        if !left_hm.is_empty() && !right_hm.is_empty() {
            for (left_k, left_v) in left_hm {
                for (right_k, right_v) in right_hm.clone() {
                    let new_k = vec![left_k, right_k];

                    if let Some(hm) = cache.get(&new_k).cloned() {
                        for (target, steps) in hm {
                            let current_hm = cache.entry(molecules.clone()).or_default();

                            let current_min =
                                current_hm.get(&target).cloned().unwrap_or(usize::MAX);
                            let new_min = left_v + right_v + steps;
                            println!(
                                "molecules = {molecules:?}; target={target:?}, current_min={current_min}, new_min ={new_min}"
                            );
                            current_hm.insert(target, current_min.min(new_min));
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::{Map, parse_map, parse_molecules};
    use rstest::*;
    #[fixture]
    fn fixture() -> Map<'static> {
        let (_, map) = parse_map(
            r#"e => H
e => O
H => HO
H => OH
O => HH"#,
        )
        .unwrap();
        map
    }

    #[rstest]
    #[case("HOH", 3)]
    #[case("HOHOHO", 6)]
    fn test_find_min(fixture: Map<'static>, #[case] m: &str, #[case] expected: usize) {
        let (_, molecules) = parse_molecules(m).unwrap();
        assert_eq!(find_min(fixture, molecules), expected);
    }
}
