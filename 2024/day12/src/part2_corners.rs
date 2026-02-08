use super::{adj4, parse, Coord};
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let grid = parse(_input);
    let mut visited = Grid::<bool>::new(grid.rows(), grid.cols());

    grid.indexed_iter().fold(0, |mut acc, ((i, j), c)| {
        if !visited[(i, j)] {
            let mut region: Vec<Coord> = vec![(i, j)];
            visited[(i, j)] = true;
            let mut corners = 0;
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
    for (x, y) in [
        (adjs.up, adjs.left),
        (adjs.up, adjs.right),
        (adjs.down, adjs.left),
        (adjs.down, adjs.right),
    ] {
        let a = x.is_some_and(|x| grid[x] == c);
        let b = y.is_some_and(|y| grid[y] == c);

        if (a && b && grid[(x.unwrap().0, y.unwrap().1)] != c) || (!a && !b) {
            *corners += 1;
        }
    }
    for adj in adj4(i, j, grid.rows(), grid.cols())
        .into_iter()
        .flatten()
        .filter(|coord| grid[*coord] == c)
    {
        if !visited[adj] {
            region.push(adj);
            visited[adj] = true;
            travel(adj.0, adj.1, c, region, grid, visited, corners)
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
