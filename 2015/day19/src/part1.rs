use crate::parsing::parse_input;
use std::collections::{HashMap, HashSet};

pub fn process(_input: &str) -> usize {
    let (map, molecules) = parse_input(_input);
    let cmap = compress_map(map);
    distinct(cmap, molecules)
}

fn distinct(cmap: HashMap<&str, Vec<Vec<&str>>>, molecules: Vec<&str>) -> usize {
    let mut set: HashSet<Vec<&str>> = HashSet::new();
    for (i, m) in molecules.iter().enumerate() {
        if let Some(replacements) = cmap.get(m) {
            for replacement in replacements {
                let mut new_molecules = molecules[0..i].to_vec();
                new_molecules.extend(&replacement[..]);
                new_molecules.extend(&molecules[i + 1..]);
                set.insert(new_molecules);
            }
        }
    }
    set.len()
}

fn compress_map<'a>(map: Vec<(&'a str, Vec<&'a str>)>) -> HashMap<&'a str, Vec<Vec<&'a str>>> {
    map.into_iter().fold(HashMap::new(), |mut acc, (k, v)| {
        acc.entry(k).or_default().push(v);
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::{parse_map, parse_molecules};
    use rstest::*;
    #[fixture]
    fn fixture() -> HashMap<&'static str, Vec<Vec<&'static str>>> {
        let (_, map) = parse_map(
            r#"H => HO
H => OH
O => HH"#,
        )
        .unwrap();
        compress_map(map)
    }

    #[rstest]
    #[case("HOH", 4)]
    #[case("HOHOHO", 7)]
    fn test_distinct(
        fixture: HashMap<&str, Vec<Vec<&str>>>,
        #[case] m: &str,
        #[case] expected: usize,
    ) {
        let (_, molecules) = parse_molecules(m).unwrap();
        assert_eq!(distinct(fixture, molecules), expected);
    }
}
