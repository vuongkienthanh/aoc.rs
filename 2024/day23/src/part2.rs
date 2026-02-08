use std::collections::{HashMap, HashSet};

type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;

pub fn process(_input: &str) -> String {
    let mut graph = _input
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .fold(Graph::new(), |mut acc, ele| {
            acc.entry(ele.0).or_default().insert(ele.1);
            acc.entry(ele.1).or_default().insert(ele.0);
            acc
        });

    let mut ans: Vec<&str> = vec![];

    for k in graph.keys().cloned().collect::<Vec<_>>() {
        if graph.get(&k).unwrap().len() > ans.len() {
            // build matrix
            let mut current = graph
                .get(&k)
                .unwrap()
                .iter()
                .map(|x| (*x, graph.get(x).unwrap() & graph.get(&k).unwrap()))
                .collect::<HashMap<_, _>>();

            // condense matrix
            let mut min_k = current
                .iter()
                .min_by_key(|x| x.1.len())
                .map(|(k, _)| *k)
                .unwrap();

            while current.iter().any(|(_, v)| v.len() + 1 != current.len()) {
                current.remove(min_k);
                for v in current.values_mut() {
                    v.remove(min_k);
                }
                min_k = current
                    .iter()
                    .min_by_key(|x| x.1.len())
                    .map(|(k, _)| *k)
                    .unwrap();
            }

            // check answer
            let (first_k, first_v) = current.iter().next().unwrap();
            if first_v.len() + 2 > ans.len() {
                ans = first_v
                    .iter()
                    .chain(Some(first_k))
                    .chain(Some(&k))
                    .cloned()
                    .collect::<Vec<_>>();
            }
        }

        // reduce graph as k becomes reduntdant
        for k2 in graph.remove(k).unwrap() {
            graph.get_mut(&k2).unwrap().remove(k);
        }
    }

    ans.sort_unstable();
    ans.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;
        assert_eq!(process(input), "co,de,ka,ta".to_string());
    }
}
