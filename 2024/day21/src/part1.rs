use std::collections::HashMap;

use crate::{directional_paths, numeric_paths, DPair, Dir, NPair, Num};

pub fn process(_input: &str) -> usize {
    let numeric_paths = numeric_paths();
    let directional_paths = directional_paths();
    _input
        .trim()
        .lines()
        .map(|line| {
            // depressurized
            let to_move = Some(Num::A)
                .into_iter()
                .chain(line.chars().map(Num::from))
                .collect::<Vec<_>>()
                .windows(2)
                .map(|v| NPair(v[0], v[1]))
                .collect::<Vec<_>>();
            let sequence = routes_to_key(to_move, &numeric_paths);

            // radiation
            let to_move_1 = sequence.into_iter().map(key_to_route).collect::<Vec<_>>();
            let sequence_1 = to_move_1
                .into_iter()
                .flat_map(|route| routes_to_key(route, &directional_paths))
                .collect::<Vec<_>>();

            // -40 degree
            let to_move_2 = sequence_1.into_iter().map(key_to_route).collect::<Vec<_>>();
            let sequence_2 = to_move_2
                .into_iter()
                .flat_map(|route| routes_to_key(route, &directional_paths))
                .collect::<Vec<_>>();

            // human
            let code_num = line[..3].parse::<usize>().unwrap();
            let min_len = sequence_2.into_iter().map(|x| x.len()).min().unwrap();
            code_num * min_len
        })
        .sum()
}

fn routes_to_key<T>(path: Vec<T>, path_map: &HashMap<T, Vec<Vec<Dir>>>) -> Vec<Vec<Dir>>
where
    T: Eq + std::hash::Hash, //NPair or DPair
{
    let mut routes: Vec<Vec<Dir>> = vec![vec![]];
    for pair in path {
        let mut new_routes = vec![];
        for possible_route in path_map.get(&pair).unwrap() {
            for current_route in &routes {
                let mut route = current_route.clone();
                route.extend_from_slice(possible_route);
                route.push(Dir::A);
                new_routes.push(route);
            }
        }
        routes = new_routes;
    }

    routes
}

fn key_to_route(keypress: Vec<Dir>) -> Vec<DPair> {
    Some(Dir::A)
        .into_iter()
        .chain(keypress)
        .collect::<Vec<_>>()
        .windows(2)
        .map(|v| DPair(v[0], v[1]))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"
029A
980A
179A
379A
456A
"#;
        assert_eq!(process(input), 126384);
    }
}
