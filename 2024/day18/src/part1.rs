use grid::Grid;
use std::collections::BinaryHeap;

type Coord = (usize, usize);

struct State {
    coord: Coord,
    len: usize,
}
impl Eq for State {}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.len.cmp(&self.len)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.len.cmp(&self.len))
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

#[derive(Default, PartialEq, Eq)]
enum CellType {
    #[default]
    Empty,
    Block,
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
    let mut costmap = Grid::new(grid.rows(), grid.cols());
    costmap.fill(usize::MAX);

    let start = (0, 0);
    let end = (size - 1, size - 1);

    let mut heap = BinaryHeap::from([State {
        coord: start,
        len: 1,
    }]);
    while let Some(State { coord, len }) = heap.pop() {
        for new_coord in adj(coord, size)
            .into_iter()
            .filter(|coord| grid[*coord] == CellType::Empty)
        {
            if new_coord == end {
                return len;
            }
            let new_len = len + 1;
            if new_len < costmap[new_coord] {
                costmap[new_coord] = new_len;
                heap.push(State {
                    coord: new_coord,
                    len: new_len,
                });
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
        assert_eq!(process(input, 7, 12), 22);
    }
}
