use super::{adj4, parse, CoordKey};
use grid::Grid;
use std::collections::HashMap;

pub fn process(_input: &str) -> usize {
    let grid = parse(_input);
    let mut visited = Grid::<bool>::new(grid.rows(), grid.cols());

    grid.indexed_iter().fold(0, |mut acc, ((i, j), c)| {
        if !visited[(i, j)] {
            let mut region = HashMap::from([(CoordKey { i, j }, 4)]);
            visited[(i, j)] = true;
            travel(i, j, *c, &mut region, &grid, &mut visited);

            let area = region.len();
            let perimeter = region.into_values().sum::<usize>();
            acc += area * perimeter;
        }
        acc
    })
}

fn travel(
    i: usize,
    j: usize,
    c: char,
    region: &mut HashMap<CoordKey, usize>,
    grid: &Grid<char>,
    visited: &mut Grid<bool>,
) {
    for adj in adj4(i, j, grid.rows(), grid.cols())
        .into_iter()
        .flatten()
        .filter(|coord| grid[*coord] == c)
    {
        *region.entry(CoordKey { i: adj.0, j: adj.1 }).or_insert(4) -= 1;
        if !visited[adj] {
            visited[adj] = true;
            travel(adj.0, adj.1, c, region, grid, visited)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
        assert_eq!(process(input), 1930);
    }
    #[test]
    fn test_process2() {
        let input = r#"AAAA
BBCD
BBCC
EEEC"#;
        assert_eq!(process(input), 140);
    }
    #[test]
    fn test_process3() {
        let input = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;
        assert_eq!(process(input), 772);
    }
}
