use super::{adj4, parse, Adj, Coord};
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let grid = parse(_input);
    let mut visited = Grid::<bool>::new(grid.rows(), grid.cols());

    grid.indexed_iter().fold(0, |mut acc, ((i, j), c)| {
        if !visited[(i, j)] {
            let mut region: Vec<Coord> = vec![(i, j)];
            let mut corners = 0;
            visited[(i, j)] = true;
            travel(i, j, *c, &mut region, &grid, &mut visited, &mut corners);

            let area = region.len();
            acc += area * corners;
        }
        acc
    })
}

fn travel(
    i: usize,
    j: usize,
    c: char,
    region: &mut Vec<Coord>,
    grid: &Grid<char>,
    visited: &mut Grid<bool>,
    corners: &mut usize,
) {
    let adjs = adj4(i, j, grid.rows(), grid.cols());
    let mut to_check = vec![];
    if let Some(x) = adjs.up {
        if let Some(y) = adjs.left {
            to_check.push((x, y));
        }
        if let Some(y) = adjs.right {
            to_check.push((x, y));
        }
    }
    if let Some(x) = adjs.down {
        if let Some(y) = adjs.left {
            to_check.push((x, y));
        }
        if let Some(y) = adjs.right {
            to_check.push((x, y));
        }
    }
    for (x, y) in to_check {
        if grid[(x.0, y.1)] != c {
            *corners += 1;
        }
        if grid[x] != c && grid[y] != c {
            *corners += 1;
        }
    }

    for adj in adjs.into_iter().flatten().filter(|coord| grid[*coord] == c) {
        if !visited[adj] {
            region.push(adj);
            visited[adj] = true;
            travel(adj.0, adj.1, c, region, grid, visited, corners);
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
        assert_eq!(process(input), 1206);
    }
    #[test]
    fn test_process2() {
        let input = r#"AAAA
BBCD
BBCC
EEEC"#;
        assert_eq!(process(input), 80);
    }
    #[test]
    fn test_process3() {
        let input = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;
        assert_eq!(process(input), 236);
    }
    #[test]
    fn test_process4() {
        let input = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;
        assert_eq!(process(input), 436);
    }
    #[test]
    fn test_process5() {
        let input = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;
        assert_eq!(process(input), 368);
    }
}
