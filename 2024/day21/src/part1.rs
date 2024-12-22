use std::collections::HashMap;

use crate::{directional_paths, numeric_paths, DirectionPair, Directional, Numeric, NumericPair};

type Keypress = Vec<Directional>;

pub fn process(_input: &str) -> usize {
    let numeric_paths = numeric_paths();
    let directional_paths = directional_paths();
    _input
        .lines()
        .map(|line| {
            let code_num = line[..3].parse::<usize>().unwrap();

            // depressurized
            let to_press = line.chars().map(Numeric::from).collect::<Vec<_>>();
            println!("to_press = {to_press:?}");
            let to_move = Some(Numeric::A)
                .into_iter()
                .chain(to_press)
                .collect::<Vec<_>>()
                .windows(2)
                .map(|v| NumericPair(v[0], v[1]))
                .collect::<Vec<_>>();
            let sequence = routes_to_keypress(to_move, &numeric_paths);

            // radiation
            let to_press_1 = flatten(sequence);
            println!("to_press_1 = {to_press_1:?}");
            let to_move_1 = to_press_1
                .into_iter()
                .map(keypress_to_route)
                .collect::<Vec<_>>();
            let sequence_1 = to_move_1
                .into_iter()
                .map(|route| routes_to_keypress(route, &directional_paths))
                .collect::<Vec<_>>();

            // -40 degree
            let to_press_2 = sequence_1.into_iter().flat_map(flatten).collect::<Vec<_>>();
            for r in &to_press_2 {
                println!("to_press_2 = {r:?}");
            }
            let to_move_2 = to_press_2
                .into_iter()
                .map(keypress_to_route)
                .collect::<Vec<_>>();
            let sequence_2 = to_move_2
                .into_iter()
                .map(|route| routes_to_keypress(route, &directional_paths))
                .collect::<Vec<_>>();

            // human
            let to_press_3 = sequence_2.into_iter().flat_map(flatten).collect::<Vec<_>>();
            for r in &to_press_3 {
                if *r
                    == code_to_vec(
                        "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
                    )
                {
                    println!("r = {r:?}");
                }
            }

            code_num
        })
        .sum()
}

fn flatten(routes: Vec<Vec<Vec<Directional>>>) -> Vec<Vec<Directional>> {
    let mut res: Vec<Vec<Directional>> = vec![vec![]];
    for origin_pair in routes {
        let mut new_routes = vec![];
        for current_route in res {
            for possible_keypress in &origin_pair {
                let mut new_route = current_route.clone();
                new_route.extend_from_slice(possible_keypress);
                new_routes.push(new_route);
            }
        }
        res = new_routes;
    }
    res
}

fn routes_to_keypress<T>(routes: Vec<T>, path_map: &HashMap<T, Vec<Keypress>>) -> Vec<Vec<Keypress>>
where
    T: Eq + std::hash::Hash,
{
    routes
        .into_iter()
        .map(|pair| {
            path_map
                .get(&pair)
                .cloned()
                .unwrap()
                .into_iter()
                .map(|mut x| {
                    x.push(Directional::A);
                    x
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn keypress_to_route(keypress: Vec<Directional>) -> Vec<DirectionPair> {
    Some(Directional::A)
        .into_iter()
        .chain(keypress)
        .collect::<Vec<_>>()
        .windows(2)
        .map(|v| DirectionPair(v[0], v[1]))
        .collect::<Vec<_>>()
}

fn code_to_vec(code: &str) -> Vec<Directional> {
    code.chars().map(Directional::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"029A
"#;
        assert_eq!(process(input), 126384);
    }
}
// 980A
// 179A
// 456A
// 379A
