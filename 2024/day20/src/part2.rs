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
            for size in 2..21 {
                for jump_time in adjx(c, size, grid)
                    .into_iter()
                    .filter_map(|c| time_used_map.get(&c.into()))
                    .filter(|jump_time| jump_time > &used_time)
                {
                    *acc.entry(jump_time - used_time - size).or_default() += 1;
                }
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
        assert_eq!(res[&50], 32);
        assert_eq!(res[&52], 31);
        assert_eq!(res[&54], 29);
        assert_eq!(res[&56], 39);
        assert_eq!(res[&58], 25);
        assert_eq!(res[&60], 23);
        assert_eq!(res[&62], 20);
        assert_eq!(res[&64], 19);
        assert_eq!(res[&66], 12);
        assert_eq!(res[&68], 14);
        assert_eq!(res[&70], 12);
        assert_eq!(res[&72], 22);
        assert_eq!(res[&74], 4);
        assert_eq!(res[&76], 3);
    }
}
