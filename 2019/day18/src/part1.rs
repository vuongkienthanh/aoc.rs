use crate::{Point, build_graph, parse};
use std::collections::BTreeMap as Map;

type State = Map<(u32, Point), usize>;

pub fn process(_input: &str) -> usize {
    let (grid, start, keys) = parse(_input);
    let graph = build_graph(&keys, &[start], &grid);

    let mut current = State::from([((0, start), 0)]);

    for _ in 0..keys.len() {
        let mut new = State::new();
        for ((has_keys, loc), step) in current {
            for ((r, c), doors, add_step) in graph.get(&loc).unwrap() {
                let key = 1 << (grid[*r][*c] - b'a') as u32;
                if has_keys & key == 0 && (doors & has_keys == *doors) {
                    let mut has_keys = has_keys;
                    has_keys |= key;
                    new.entry((has_keys, (*r, *c)))
                        .and_modify(|x| *x = (*x).min(step + add_step))
                        .or_insert(step + add_step);
                }
            }
        }

        current = new;
    }

    current.into_values().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        r#"#########
#b.A.@.a#
#########"#,
        8
    )]
    #[case(
        r#"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"#,
        86
    )]
    #[case(
        r#"########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################"#,
        132
    )]
    #[case(
        r#"#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################"#,
        136
    )]
    #[case(
        r#"########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################"#,
        81
    )]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
