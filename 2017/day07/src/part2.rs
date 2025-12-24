use crate::parsing::parse_input;
use crate::{Node, build_map};
use fxhash::FxHashMap as Map;

type NodeMap<'a> = Map<&'a str, Node<'a>>;
type WeightCache<'a> = Map<&'a str, usize>;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    let map = build_map(input);
    let mut weight_cache = WeightCache::default();

    // find root
    let mut node = map
        .iter()
        .find_map(|(k, v)| v.parent.is_none().then_some(*k))
        .unwrap();
    let mut target_weight = usize::MAX;

    'a: loop {
        if let Some((new_node, new_target_weight)) =
            find_unbalanced_child(&map, node, &mut weight_cache)
        {
            node = new_node;
            target_weight = new_target_weight;
        } else {
            let diff = target_weight
                .checked_signed_diff(get_weight(&map, node, &mut weight_cache))
                .unwrap();
            break 'a map.get(&node).unwrap().weight.strict_add_signed(diff);
        }
    }
}

fn get_weight<'a>(map: &NodeMap<'a>, node: &'a str, weight_cache: &mut WeightCache<'a>) -> usize {
    if let Some(w) = weight_cache.get(&node) {
        *w
    } else {
        let n = map.get(&node).unwrap();
        let w = n.weight
            + n.children
                .iter()
                .map(|c| get_weight(map, c, weight_cache))
                .sum::<usize>();
        weight_cache.insert(node, w);
        w
    }
}

fn is_balance<'a>(
    map: &NodeMap<'a>,
    node: &'a str,
    weight_cache: &mut WeightCache<'a>,
) -> bool {
         map
            .get(&node)
            .unwrap()
            .children
            .iter()
            .map(|x| get_weight(map, x, weight_cache))
            .collect::<Vec<usize>>()
            .windows(2)
            .all(|x| x[0] == x[1])
    }
}

fn find_unbalanced_child<'a>(
    map: &NodeMap<'a>,
    node: &'a str,
    weight_cache: &mut WeightCache<'a>,
    balance_cache: &mut BalanceCache<'a>,
) -> Option<(&'a str, usize)> {
    fn inner<'a>(
        map: &NodeMap<'a>,
        node: &'a str,
        weight_cache: &mut WeightCache<'a>,
        balance_cache: &mut BalanceCache<'a>,
    ) -> Option<(&'a str, usize) {
        let children = &map.get(&node).unwrap().children;
        match children.len() {
            2 => match (
                is_balance(map, children[0], weight_cache, balance_cache),
                is_balance(map, children[1], weight_cache, balance_cache),
            ) {
                (false, true) => inner(map, children[0], weight_cache, balance_cache),
                (true, false) => inner(map, children[1], weight_cache, balance_cache),
                _ => panic!("one tower must be unbalanced"),
            },
            _ => {
                let mut counting: Map<usize, Vec<&str>> = Map::default();
                for c in children {
                    let w = get_weight(map, c, weight_cache);
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
    if is_balance(map, node, weight_cache, balance_cache) {
        None
    } else {
        inner(map, node, weight_cache, balance_cache)
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
