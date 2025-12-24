pub mod parsing;
pub mod part1;
pub mod part2;

use fxhash::FxHashMap as Map;
use parsing::Item;

#[derive(Debug)]
struct Node<'a> {
    weight: usize,
    parent: Option<&'a str>,
    children: Vec<&'a str>,
}

fn build_map<'a>(input: Vec<Item<'a>>) -> Map<&'a str, Node<'a>> {
    let mut map: Map<&'a str, Node<'a>> = Map::default();
    for item in input {
        match item {
            Item::Name(name, weight) => {
                map.entry(name)
                    .and_modify(|node| node.weight = weight)
                    .or_insert(Node {
                        weight,
                        parent: None,
                        children: vec![],
                    });
            }
            Item::Assign(name, weight, children) => {
                map.entry(name)
                    .and_modify(|node| {
                        node.weight = weight;
                        node.children = children.clone();
                    })
                    .or_insert(Node {
                        weight: weight,
                        parent: None,
                        children: children.clone(),
                    });
                for c in children {
                    map.entry(c)
                        .and_modify(|node| node.parent = Some(name))
                        .or_insert(Node {
                            weight: 0,
                            parent: Some(name),
                            children: vec![],
                        });
                }
            }
        }
    }
    map
}
