use crate::parsing::parse_input;
use fxhash::FxHashMap;

#[derive(Debug)]
struct Node<'a> {
    parents: Vec<&'a str>,
    child: Option<&'a str>,
}

pub fn process<'a>(_input: &'a str) -> usize {
    let input = parse_input(_input);

    let mut map: FxHashMap<&'a str, Node<'a>> = FxHashMap::default();

    for (a, b) in input {
        map.entry(a)
            .and_modify(|n| n.parents.push(b))
            .or_insert(Node {
                parents: vec![b],
                child: None,
            });
        map.entry(b)
            .and_modify(|n| {
                assert!(n.child.is_none());
                n.child = Some(a);
            })
            .or_insert(Node {
                parents: vec![],
                child: Some(a),
            });
    }

    let mut ans = 1;

    let mut current: Vec<(&str, &str)> = vec![("YOU", map.get("YOU").unwrap().child.unwrap())];

    'a: loop {
        let mut new: Vec<(&str, &str)> = vec![];

        ans += 1;
        for (a, b) in current {
            for c in map
                .get(b)
                .unwrap()
                .parents
                .iter()
                .chain(map.get(b).unwrap().child.as_ref())
                .filter(|p| p != &&a)
            {
                if c == &"SAN" {
                    break 'a;
                }
                new.push((b, *c));
            }
        }

        current = new;
    }

    ans - 2
}
