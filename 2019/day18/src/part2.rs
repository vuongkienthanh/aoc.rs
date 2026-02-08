use crate::{Point, build_graph, parse};
use std::collections::BTreeMap as Map;

type State = Map<(u32, [Point; 4]), usize>;

pub fn process(_input: &str) -> usize {
    let (mut grid, start, keys) = parse(_input);
    let starts = transfrom_grid(&mut grid, start);
    let graph = build_graph(&keys, &starts, &grid);
    let mut current = State::from([((0, starts), 0)]);
    for k in 1..=keys.len() {
        println!("{k}/{}", keys.len());
        let mut new = State::new();
        for ((has_keys, pos4), step) in current {
            for (i, loc) in pos4.iter().enumerate() {
                for ((r, c), doors, add_step) in graph.get(loc).unwrap() {
                    let key = 1 << (grid[*r][*c] - b'a') as u32;
                    if has_keys & key == 0 && doors & has_keys == *doors {
                        let mut has_keys = has_keys;
                        has_keys |= key;
                        let mut pos4 = pos4;
                        pos4[i] = (*r, *c);
                        new.entry((has_keys, pos4))
                            .and_modify(|x| *x = (*x).min(step + add_step))
                            .or_insert(step + add_step);
                    }
                }
            }
        }
        current = new;
    }

    current.into_values().min().unwrap()
}

fn transfrom_grid(grid: &mut [Vec<u8>], (r, c): Point) -> [Point; 4] {
    grid[r][c] = b'#';
    grid[r - 1][c] = b'#';
    grid[r + 1][c] = b'#';
    grid[r][c - 1] = b'#';
    grid[r][c + 1] = b'#';

    [
        (r - 1, c - 1),
        (r - 1, c + 1),
        (r + 1, c - 1),
        (r + 1, c + 1),
    ]
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        r#"#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######"#,
        8
    )]
    #[case(
        r#"#############
#DcBa.#.GhKl#
#.###...#I###
#e#d#.@.#j#k#
###C#...###J#
#fEbA.#.FgHi#
#############"#,
        32
    )]
    #[case(
        r#"#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba...BcIJ#
#####.@.#####
#nK.L...G...#
#M###N#H###.#
#o#m..#i#jk.#
#############"#,
        72
    )]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
