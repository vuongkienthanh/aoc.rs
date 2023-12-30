use std::cmp::Ordering;
use std::collections::BinaryHeap;

type Coord = (usize, usize);

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Route {
    heat_loss: u32,
    nodes: Vec<Coord>,
    last_dir: Direction,
}

impl PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.heat_loss.cmp(&self.heat_loss))
    }
}
impl Ord for Route {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}
impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        self.heat_loss.eq(&other.heat_loss)
    }
}
impl Eq for Route {}

#[derive(Debug)]
struct Puzzle {
    input: Vec<Vec<u32>>,
    heap: BinaryHeap<Route>,
    max_rows: usize,
    max_cols: usize,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        Self {
            input: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
            heap: BinaryHeap::new(),
            max_rows: input.lines().count(),
            max_cols: input.lines().next().unwrap().len(),
        }
    }
    fn num_at(&self, loc: Coord) -> u32 {
        *self.input.get(loc.0).unwrap().get(loc.1).unwrap()
    }
    fn new_astar(&self, old: &Route) -> Vec<Route> {
        let mut ret = vec![];
        let old_row = old.nodes.last().unwrap().0;
        let old_col = old.nodes.last().unwrap().1;
        let mut addition_heat_loss = 0;
        let mut new_nodes = old.nodes.clone();

        match old.last_dir {
            Direction::Up => {
                for i in 1..=3 {
                    if let Some(row) = old_row.checked_sub(i) {
                        let new_coord = (row, old_col);
                        if old.nodes.contains(&new_coord) {
                            break;
                        } else {
                            new_nodes.push(new_coord);
                            addition_heat_loss += self.num_at(new_coord);
                            for dir in [Direction::Left, Direction::Right] {
                                ret.push(Route {
                                    heat_loss: old.heat_loss + addition_heat_loss.clone(),
                                    nodes: new_nodes.clone(),
                                    last_dir: dir,
                                });
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
            Direction::Down => {
                let row_start = (old_row + 1).min(self.max_rows - 1);
                let row_end = (old_row + 3).min(self.max_rows - 1);
                for row in row_start..=row_end {
                    let new_coord = (row, old_col);
                    if old.nodes.contains(&new_coord) {
                        break;
                    } else {
                        new_nodes.push(new_coord);
                        addition_heat_loss += self.num_at(new_coord);
                        for dir in [Direction::Left, Direction::Right] {
                            ret.push(Route {
                                heat_loss: old.heat_loss + addition_heat_loss.clone(),
                                nodes: new_nodes.clone(),
                                last_dir: dir,
                            });
                        }
                    }
                }
            }
            Direction::Left => {
                for i in 1..=3 {
                    if let Some(col) = old_col.checked_sub(i) {
                        let new_coord = (old_row, col);
                        if old.nodes.contains(&new_coord) {
                            break;
                        } else {
                            new_nodes.push(new_coord);
                            addition_heat_loss += self.num_at(new_coord);
                            for dir in [Direction::Up, Direction::Down] {
                                ret.push(Route {
                                    heat_loss: old.heat_loss + addition_heat_loss.clone(),
                                    nodes: new_nodes.clone(),
                                    last_dir: dir,
                                });
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
            Direction::Right => {
                let col_start = (old_col + 1).min(self.max_cols - 1);
                let col_end = (old_col + 3).min(self.max_cols - 1);
                for col in col_start..=col_end {
                    let new_coord = (old_row, col);
                    if old.nodes.contains(&new_coord) {
                        break;
                    } else {
                        new_nodes.push(new_coord);
                        addition_heat_loss += self.num_at(new_coord);
                        for dir in [Direction::Up, Direction::Down] {
                            ret.push(Route {
                                heat_loss: old.heat_loss + addition_heat_loss.clone(),
                                nodes: new_nodes.clone(),
                                last_dir: dir,
                            });
                        }
                    }
                }
            }
        }
        ret
    }

    fn run(&mut self, start: Coord, end: Coord) -> usize {
        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            self.heap.push(Route {
                heat_loss: 0,
                nodes: vec![start],
                last_dir: dir,
            });
        }
        let mut i = 0;
        while let Some(astar) = self.heap.pop() {
            if astar.nodes.last().unwrap() == &end {
                return astar.heat_loss as usize;
            } else {
                for new_astar in self.new_astar(&astar) {
                    // dbg!(&new_astar);
                    self.heap.push(new_astar);
                }
            }
            dbg!(&self.heap);
            i += 1;
            if i == 10 {
                break;
            }
        }

        panic!("cant find end coord")
    }
}

pub fn process(input: &str) -> usize {
    let max_rows = input.lines().count();
    let max_cols = input.lines().next().unwrap().len();
    Puzzle::new(input).run((0, 0), (max_rows - 1, max_cols - 1))
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

