use crate::adj::adj4;
use crate::parsing::{G, Item, Point};
use std::collections::HashMap;

fn walk(from: char, g: &G, number_locations: &HashMap<char, Point>) -> HashMap<char, usize> {
    let mut ans = HashMap::new();
    let from_p = number_locations.get(&from).cloned().unwrap();
    let needed_numbers = number_locations
        .iter()
        .filter(|(k, _)| *k != &from)
        .map(|(k,v)| (*k, *v))
        .collect::<HashMap<char, Point>>();

    type State = (Point, Point, Vec<Point>);

    let mut v: Vec<State> = vec![];

    for adj in adj4(from_p) {
        match g[adj] {
            Item::Wall => continue,
            Item::Space => {
                v.push((from_p, adj, vec![from_p]));
            }
            Item::Cross => {
                v.push((from_p, adj, vec![from_p, adj]));
            }
        }
        if let Some((k, _)) = needed_numbers.iter().find(|(_, v)| *v == &adj) {
            ans.insert(*k, 1);
        }
    }

    let mut step = 1;

    while ans.len() < needed_numbers.len() {
        let mut new_v = vec![];
        step += 1;
        while let Some((prev, curr, path)) = v.pop() {
            for adj in adj4(curr) {
                if adj == prev {
                    continue;
                }
                match g[adj] {
                    Item::Wall => continue,
                    Item::Space => {
                        if adj != from_p {
                            new_v.push((curr, adj, path));
                            if let Some((k, _)) = needed_numbers.iter().find(|(_, v)| *v == &adj) {
                                if !ans.contains_key(k) {
                                    ans.insert(*k, step);
                                }
                            }
                        }
                    }
                    Item::Cross => {
                        if !path.contains(&adj) {
                            let mut new_path = path.clone();
                            new_path.push(adj);
                            new_v.push((curr, adj, new_path));
                            if let Some((k, _)) = needed_numbers.iter().find(|(_, v)| *v == &adj) {
                                if !ans.contains_key(k) {
                                    ans.insert(*k, step);
                                }
                            }
                        }
                    }
                }
            }
        }

        v.extend(new_v);
    }

    ans
}