use super::{adj4, parse, Coord};
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let grid = parse(_input);
    let mut visited = Grid::<bool>::new(grid.rows(), grid.cols());

    grid.indexed_iter().fold(0, |mut acc, ((i, j), c)| {
        if !visited[(i, j)] {
            let mut region: Vec<Coord> = vec![(i, j)];
            visited[(i, j)] = true;
            travel(i, j, *c, &mut region, &grid, &mut visited);

            let area = region.len();
            let sides = count_sides(&region);
            acc += area * sides;
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
) {
    for adj in adj4(i, j, grid.rows(), grid.cols())
        .into_iter().flatten()
        .filter(|coord| grid[*coord] == c)
    {
        if !visited[adj] {
            region.push(adj);
            visited[adj] = true;
            travel(adj.0, adj.1, c, region, grid, visited)
        }
    }
}

fn count_sides(region: &[Coord]) -> usize {
    let bounds = get_bounds(region);
    let (up_bound, down_bound, left_bound, right_bound) = bounds;
    let (mut up_row, mut down_row, mut left_col, mut right_col) = get_borders(region, bounds);

    let (mut up_sides, mut down_sides, mut left_sides, mut right_sides) = (
        count_horizontal(&mut up_row),
        count_horizontal(&mut down_row),
        count_vertical(&mut left_col),
        count_vertical(&mut right_col),
    );

    for i in up_bound + 1..down_bound + 1 {
        let row = region
            .iter()
            .filter(|c| c.0 == i)
            .map(|c| c.1)
            .collect::<Vec<_>>();
        up_sides += count_horizontal(
            &mut row
                .iter()
                .filter(|j| !up_row.contains(j))
                .cloned()
                .collect::<Vec<_>>(),
        );
        up_row = row;
    }
    for i in (up_bound..down_bound).rev() {
        let row = region
            .iter()
            .filter(|c| c.0 == i)
            .map(|c| c.1)
            .collect::<Vec<_>>();
        down_sides += count_horizontal(
            &mut row
                .iter()
                .filter(|j| !down_row.contains(j))
                .cloned()
                .collect::<Vec<_>>(),
        );
        down_row = row;
    }
    for j in left_bound + 1..right_bound + 1 {
        let col = region
            .iter()
            .filter(|c| c.1 == j)
            .map(|c| c.0)
            .collect::<Vec<_>>();
        left_sides += count_vertical(
            &mut col
                .iter()
                .filter(|i| !left_col.contains(i))
                .cloned()
                .collect::<Vec<_>>(),
        );
        left_col = col;
    }
    for j in (left_bound..right_bound).rev() {
        let col = region
            .iter()
            .filter(|c| c.1 == j)
            .map(|c| c.0)
            .collect::<Vec<_>>();
        right_sides += count_vertical(
            &mut col
                .iter()
                .filter(|i| !right_col.contains(i))
                .cloned()
                .collect::<Vec<_>>(),
        );
        right_col = col;
    }

    up_sides + down_sides + left_sides + right_sides
}

// up, down, left, right
fn get_bounds(region: &[Coord]) -> (usize, usize, usize, usize) {
    let (mut up, mut down, mut left, mut right) = (usize::MAX, 0, usize::MAX, 0);
    for coord in region {
        up = up.min(coord.0);
        down = down.max(coord.0);
        left = left.min(coord.1);
        right = right.max(coord.1);
    }
    (up, down, left, right)
}

// up, down, left, right
fn get_borders(
    region: &[Coord],
    bounds: (usize, usize, usize, usize),
) -> (Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>) {
    let (mut up, mut down, mut left, mut right) = (vec![], vec![], vec![], vec![]);
    for coord in region {
        if coord.0 == bounds.0 {
            up.push(coord.1);
        }
        if coord.0 == bounds.1 {
            down.push(coord.1);
        }
        if coord.1 == bounds.2 {
            left.push(coord.0);
        }
        if coord.1 == bounds.3 {
            right.push(coord.0);
        }
    }
    (up, down, left, right)
}

fn count_horizontal(row: &mut [usize]) -> usize {
    if row.is_empty() {
        0
    } else {
        row.sort_unstable();
        row.windows(2).filter(|v| v[1] - v[0] > 1).count() + 1
    }
}
fn count_vertical(col: &mut [usize]) -> usize {
    if col.is_empty() {
        0
    } else {
        col.sort_unstable();
        col.windows(2).filter(|v| v[1] - v[0] > 1).count() + 1
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
