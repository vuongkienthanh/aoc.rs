use std::cmp::Ordering;
use std::collections::BinaryHeap;

type Coord = (usize, usize);

enum Direction {
    Up,
    Down,
    Left,
    Right,
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
            max_cols: input.lines().next().unwrap().len(),
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
    fn new_astar(&self, old: Astar) -> Vec<Astar> {
        let mut ret = vec![];
        match old.dir {
            Direction::Up => {
                for i in 1..4 {
                    if let Some(row) = old.loc.0.checked_sub(i) {
                        let addition_score: usize = (old.loc.0 - 1..=row)
                            .map(|j| self.num_at((j, old.loc.1)))
                            .sum();
                        for dir in [Direction::Left, Direction::Right] {
                            ret.push(Astar {
                                score: old.score + addition_score,
                                loc: (row, old.loc.1),
                                dir,
                            });
                        }
                    }
                }
            }
            Direction::Down => {
                if old.loc.0 < self.max_rows - 1 {
                    for row in (old.loc.0 + 1)..=((old.loc.0 + 3).min(self.max_rows - 1)) {
                        let addition_score: usize = ((old.loc.0 + 1)..=row)
                            .map(|j| self.num_at((j, old.loc.1)))
                            .sum();
                        for dir in [Direction::Left, Direction::Right] {
                            ret.push(Astar {
                                score: old.score + addition_score,
                                loc: (row, old.loc.1),
                                dir,
                            });
                        }
                    }
                }
            }
            Direction::Left => todo!(),
            Direction::Right => todo!(),
        }
        ret
    }
}

pub fn process(input: &str) -> usize {
    let max_rows = input.lines().count();
    let max_cols = input.lines().next().unwrap().len();
    let start_loc = (0, 0);
    let end_loc = (max_rows - 1, max_cols - 1);

    0
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
