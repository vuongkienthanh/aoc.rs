use std::cmp::Ordering;
use std::collections::BinaryHeap;

type Coord = (usize, usize);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offset(&self) -> [(isize, isize);3] {
        match self {
            Direction::Up => [(-3,0), (-2,0),(-1,0)],
            Direction::Down => [(1,0), (2,0), (3,0)],
            Direction::Left => [(0,-3), (0,-2), (0,-1)],
            Direction::Right => [(0,1),(0,2),(0,3)],
        }
    }
}

struct Astar {
    score: usize,
    loc: Coord,
    dir: Direction,
}

impl PartialOrd for Astar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.score.partial_cmp(&self.score)
    }
}
impl Ord for Astar {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}
impl PartialEq for Astar {
    fn eq(&self, other: &Self) -> bool {
        self.score.eq(&other.score)
    }
    fn ne(&self, other: &Self) -> bool {
        self.score.ne(&other.score)
    }
}
impl Eq for Astar {}

// impl Astar {
//     fn next_astar(&self, max_rows, max_cols) -> Vec<Self> {
//
//     }
// }
//

struct Puzzle<'a> {
    input: &'a str,
    heap: BinaryHeap<Astar>,
    max_rows: usize,
    max_cols: usize,
    start_loc: Coord,
    end_loc: Coord,
}

impl<'a> Puzzle<'a> {
    fn new(input: &'a str, start_loc: Coord, end_loc: Coord) -> Self {
        Self {
            input,
            heap: BinaryHeap::new(),
            max_rows: input.lines().count(),
            max_cols:  input.lines().next().unwrap().len(),
            start_loc,
            end_loc,
        }
    }
fn num_at(&self, loc: Coord) -> usize {
    self.input
        .lines()
        .nth(loc.0)
        .unwrap()
        .chars()
        .nth(loc.1)
        .unwrap()
        .to_digit(10)
        .unwrap() as usize
}
}

pub fn process(input: &str) -> usize {
    let start_score = num_at(input, 0, 0);
    let max_rows = input.lines().count();
    let max_cols = input.lines().next().unwrap().len();
    let end_loc = (max_rows - 1, max_cols - 1);

    let mut heap: BinaryHeap<Astar> = BinaryHeap::new();

    for i in 1..4 {
    heap.push(Astar {
        score: num_at(input, 0, i),
        loc: start_loc,
        dir: Direction::Down,
    });
    heap.push(Astar {
        score: start_score,
        loc: start_loc,
        dir: Direction::Right,
    });

    }

    while let Some(route) = heap.pop() {
        if route.loc == end_loc {
            return route.score
        }
        for 

    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;
        assert_eq!(process(input), 102);
    }
}
