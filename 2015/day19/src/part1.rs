use crate::parsing::{Map, parse_input};
use std::collections::HashSet;
pub fn process(_input: &str) -> usize {
    let (_rest, (map, molecules)) = parse_input(_input).unwrap();
    assert!(_rest.is_empty());
    distinct(&map, molecules)
}

fn distinct<'a>(map: &Map<'a>, molecules: Vec<&'a str>) -> usize {
    let mut set: HashSet<String> = HashSet::new();
    for (i, m) in molecules.iter().enumerate() {
        if let Some(possible_replacements) = map.get(m) {
            for replacement in possible_replacements {
                let mut new_molecules = molecules.clone();
                new_molecules[i] = replacement;
                set.insert(new_molecules.into_iter().collect());
            }
        }
    }
    set.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::{Map, parse_map, parse_molecules};
    use rstest::*;
    #[fixture]
    fn fixture() -> Map<'static> {
        let (_, map) = parse_map(
            r#"H => HO
H => OH
O => HH"#,
        )
        .unwrap();
        map
    }

    #[rstest]
    #[case("HOH", 4)]
    #[case("HOHOHO", 7)]
    fn test_distinct(fixture: Map<'static>, #[case] m: &str, #[case] expected: usize) {
        let (_, molecules) = parse_molecules(m).unwrap();
        assert_eq!(distinct(&fixture, molecules), expected);
    }
}
