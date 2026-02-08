use grid::Grid;

type Coord = (usize, usize);

#[derive(Default, PartialEq, Eq, Clone, Copy)]
enum CellType {
    #[default]
    Empty,
    White,
    Red,
    Blue,
}
fn adj(coord: Coord, size: usize) -> Vec<Coord> {
    let mut ans = vec![];
    if coord.0 > 0 {
        ans.push((coord.0 - 1, coord.1));
        if coord.1 > 0 {
            ans.push((coord.0 - 1, coord.1 - 1));
        }
        if coord.1 < size - 1 {
            ans.push((coord.0 - 1, coord.1 + 1));
        }
    }
    if coord.0 < size - 1 {
        ans.push((coord.0 + 1, coord.1));
        if coord.1 > 0 {
            ans.push((coord.0 + 1, coord.1 - 1));
        }
        if coord.1 < size - 1 {
            ans.push((coord.0 + 1, coord.1 + 1));
        }
    }
    if coord.1 > 0 {
        ans.push((coord.0, coord.1 - 1));
    }
    if coord.1 < size - 1 {
        ans.push((coord.0, coord.1 + 1));
    }
    ans
}

fn infect(coord: Coord, celltype: CellType, grid: &mut Grid<CellType>) {
    grid[coord] = celltype;
    for c in adj(coord, grid.rows()) {
        if grid[c] == CellType::White {
            infect(c, celltype, grid);
        }
    }
}

pub fn process(_input: &str, size: usize) -> String {
    let mut grid: Grid<CellType> = Grid::new(size, size);

    for coord in _input.lines() {
        let (x, y) = coord
            .split_once(',')
            .map(|x| (x.1.parse::<usize>().unwrap(), x.0.parse::<usize>().unwrap()))
            .unwrap();
        let is_red = x == grid.rows() - 1
            || y == 0
            || adj((x, y), size)
                .into_iter()
                .any(|c| grid[c] == CellType::Red);
        let is_blue = x == 0
            || y == grid.cols() - 1
            || adj((x, y), size)
                .into_iter()
                .any(|c| grid[c] == CellType::Blue);

        match (is_red, is_blue) {
            (true, true) => return format!("{y},{x}"),
            (true, false) => infect((x, y), CellType::Red, &mut grid),
            (false, true) => infect((x, y), CellType::Blue, &mut grid),
            (false, false) => grid[(x, y)] = CellType::White,
        }

        if cfg!(test) {
            println!("=============== checking {y} {x}");
            for line in grid.iter_rows() {
                for c in line {
                    match c {
                        CellType::Empty => print!("."),
                        CellType::White => print!("W"),
                        CellType::Red => print!("R"),
                        CellType::Blue => print!("B"),
                    }
                }
                println!();
            }
        }
    }

    panic!("should have an answer")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;
        assert_eq!(process(input, 7), "6,1".to_string());
    }
}
