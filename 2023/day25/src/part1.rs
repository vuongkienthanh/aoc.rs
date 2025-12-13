use std::collections::{hash_map::RandomState, HashMap, HashSet};

fn parse_input(input: &str) -> HashMap<String, HashSet<String>> {
    input.lines().fold(HashMap::new(), |mut hm, line| {
        let (left, right) = line.split_once(": ").unwrap();
        for node in right.split_ascii_whitespace() {
            hm.entry(left.to_string())
                .or_default()
                .insert(node.to_string());
            hm.entry(node.to_string())
                .or_default()
                .insert(left.to_string());
        }
        hm
    })
}

pub fn process(_input: &str) -> usize {
    let mut graph = parse_input(_input);

    let initial_len = graph.len();
    let mut external_count: HashMap<String, usize, RandomState> =
        HashMap::from_iter(graph.keys().map(|k| (k.to_string(), 0)));

    while external_count.values().sum::<usize>() != 3 {
        let max = external_count
            .iter()
            .max_by_key(|(_, v)| *v)
            .unwrap()
            .0
            .clone();
        external_count.remove(&max).unwrap();
        for n in graph.remove(&max).unwrap() {
            *external_count.get_mut(&n).unwrap() += 1;
            graph.get_mut(&n).unwrap().remove(&max);
        }
    }

    return (initial_len - external_count.len()) * external_count.len()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;
        assert_eq!(process(input), 54);
    }
}
