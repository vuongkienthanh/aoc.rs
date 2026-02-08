use crate::parsing::parse_input;
use crate::{Node, build_map};
use fxhash::FxHashMap as Map;

type NodeMap<'a> = Map<&'a str, Node<'a>>;
type Cache<'a> = Map<&'a str, usize>;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    let map = build_map(input);
    let mut cache = Cache::default();

    // find root
    let mut node = map
        .iter()
        .find_map(|(k, v)| v.parent.is_none().then_some(*k))
        .unwrap();
    let mut target_weight = usize::MAX;

    'a: loop {
        if let Some((new_node, new_target_weight)) = find_unbalanced_child(&map, node, &mut cache) {
            node = new_node;
            target_weight = new_target_weight;
        } else {
            let diff = target_weight
                .checked_signed_diff(get_weight(&map, node, &mut cache))
                .unwrap();
            break 'a map.get(&node).unwrap().weight.strict_add_signed(diff);
        }
    }
}

fn get_weight<'a>(map: &NodeMap<'a>, node: &'a str, cache: &mut Cache<'a>) -> usize {
    if let Some(w) = cache.get(&node) {
        *w
    } else {
        let n = map.get(&node).unwrap();
        let w = n.weight
            + n.children
                .iter()
                .map(|c| get_weight(map, c, cache))
                .sum::<usize>();
        cache.insert(node, w);
        w
    }
}

fn is_balance<'a>(map: &NodeMap<'a>, node: &'a str, cache: &mut Cache<'a>) -> bool {
    map.get(&node)
        .unwrap()
        .children
        .iter()
        .map(|x| get_weight(map, x, cache))
        .collect::<Vec<usize>>()
        .windows(2)
        .all(|x| x[0] == x[1])
}

fn find_unbalanced_child<'a>(
    map: &NodeMap<'a>,
    node: &'a str,
    cache: &mut Cache<'a>,
) -> Option<(&'a str, usize)> {
    fn into_child<'a>(
        map: &NodeMap<'a>,
        node: &'a str,
        cache: &mut Cache<'a>,
    ) -> Option<(&'a str, usize)> {
        let children = &map.get(&node).unwrap().children;
        match children.len() {
            2 => {
                // one of the children must be unbalanced
                if !is_balance(map, children[0], cache) {
                    into_child(map, children[0], cache)
                } else {
                    into_child(map, children[1], cache)
                }
            }
            _ => {
                let mut counting: Map<usize, Vec<&str>> = Map::default();
                for c in children {
                    let w = get_weight(map, c, cache);
                    counting.entry(w).or_default().push(c);
                }
                let node = counting
                    .iter()
                    .find_map(|(_, v)| (v.len() == 1).then_some(v[0]))
                    .unwrap();
                let target = counting
                    .into_iter()
                    .find_map(|(w, v)| (v.len() > 1).then_some(w))
                    .unwrap();
                Some((node, target))
            }
        }
    }
    if is_balance(map, node, cache) {
        None
    } else {
        into_child(map, node, cache)
    }
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
        assert_eq!(process(fixture), 60);
    }
}
