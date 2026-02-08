use crate::{adjx, mapping, parse, CellType, CoordKey};
use grid::Grid;
use std::collections::HashMap;

pub fn process(_input: &str) -> usize {
    let (start, end, grid) = parse(_input);
    let time_used_map = mapping(start, end, &grid);
    find_cheats(&grid, &time_used_map)
        .into_iter()
        .filter_map(|(k, v)| (k >= 100).then_some(v))
        .sum()
}

fn find_cheats(
    grid: &Grid<CellType>,
    time_used_map: &HashMap<CoordKey, usize>,
) -> HashMap<usize, usize> {
    time_used_map
        .iter()
        .map(|(CoordKey(x, y), time)| ((*x, *y), time))
        .fold(HashMap::new(), |mut acc, (c, used_time)| {
            for jump_time in adjx(c, 2, grid)
                .into_iter()
                .filter_map(|c| time_used_map.get(&c.into()))
                .filter(|jump_time| jump_time > &used_time)
            {
                *acc.entry(jump_time - used_time - 2).or_default() += 1;
            }
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;
        let (start, end, grid) = parse(input);
        let map = mapping(start, end, &grid);
        let res = find_cheats(&grid, &map);
        assert_eq!(res[&2], 14);
        assert_eq!(res[&4], 14);
        assert_eq!(res[&6], 2);
        assert_eq!(res[&8], 4);
        assert_eq!(res[&10], 2);
        assert_eq!(res[&12], 3);
        assert_eq!(res[&20], 1);
        assert_eq!(res[&36], 1);
        assert_eq!(res[&38], 1);
        assert_eq!(res[&40], 1);
        assert_eq!(res[&64], 1);
    }
}
