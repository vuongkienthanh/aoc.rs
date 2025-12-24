use crate::build_map;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> &str {
    let input = parse_input(_input);
    let map = build_map(input);
    map.into_iter()
        .find_map(|(k, v)| v.parent.is_none().then_some(k))
        .unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), "tknk");
    }
}
