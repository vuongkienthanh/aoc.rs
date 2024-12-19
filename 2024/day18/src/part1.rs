use grid::Grid;
use std::collections::BinaryHeap;

type Coord = (usize, usize);

struct State {
    cost: usize,
    coord: Coord,
    visited: Vec<Coord>,
}
impl Eq for State {}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}
fn adj(coord: Coord, size: usize) -> Vec<Coord> {
    let mut ans = vec![];
    if coord.0 > 0 {
        ans.push((coord.0 - 1, coord.1))
    };
    if coord.1 > 0 {
        ans.push((coord.0, coord.1 - 1))
    };
    if coord.0 < size - 1 {
        ans.push((coord.0 + 1, coord.1))
    };
    if coord.1 < size - 1 {
        ans.push((coord.0, coord.1 + 1))
    };
    ans
}
fn manhattan(coord: Coord, end: Coord) -> usize {
    end.0 - coord.0 + end.1 - coord.1
}

#[derive(Default, PartialEq, Eq)]
enum CellType {
    #[default]
    Empty,
    Block,
    White,
}

pub fn process(_input: &str, size: usize, count: usize) -> usize {
    let mut grid: Grid<CellType> = Grid::new(size, size);

    for coord in _input.lines().take(count) {
        let coord = coord
            .split_once(',')
            .map(|x| (x.1.parse::<usize>().unwrap(), x.0.parse::<usize>().unwrap()))
            .unwrap();
        grid[coord] = CellType::Block;
    }

    if cfg!(test) {
        for line in grid.iter_rows() {
            for c in line {
                match c {
                    CellType::Empty => print!("."),
                    CellType::Block => print!("#"),
                    CellType::White => (),
                }
            }
            println!();
        }
    }

    let cost_max = size * size;
    let start = (0, 0);
    let end = (size - 1, size - 1);

    let mut heap = BinaryHeap::from([State {
        cost: manhattan(start, end) + 1,
        coord: start,
        visited: vec![start],
    }]);
    while let Some(State {
        cost,
        coord,
        visited,
    }) = heap.pop()
    {
        for new_coord in adj(coord, size)
            .into_iter()
            .filter(|coord| grid[*coord] == CellType::Empty)
        {
            if new_coord == end {
                return visited.len() + 1;
            }
            if !visited.contains(&new_coord) {
                // if cfg!(test) {
                    println!("testing newcoord {new_coord:?}");
                    for (i, line) in grid.iter_rows().enumerate() {
                        for (j, c) in line.enumerate() {
                            if visited.contains(&(i, j)) {
                                print!("0");
                            } else {
                                match c {
                                    CellType::Empty => print!("."),
                                    CellType::Block => print!("#"),
                                    CellType::White => (),
                                }
                            }
                        }
                        println!();
                    }
                // }
                let mut new_visited = visited.clone();
                new_visited.push(new_coord);
                heap.push(State {
                    cost: manhattan(new_coord, end) + new_visited.len(),
                    coord: new_coord,
                    visited: new_visited,
                });
            }
        }
    }

    todo!("part1")
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
        assert_eq!(process(input, 7, 12), 22);
    }
}
