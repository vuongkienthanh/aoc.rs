use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord(usize, usize);
impl Coord {
    fn mahattan(&self, other: &Self) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Route {
    heat_loss: usize,
    mahatan: usize,
    nodes: Vec<Coord>,
    last_dir: Direction,
}

impl PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (other.heat_loss + other.mahatan).partial_cmp(&(self.heat_loss + self.mahatan))
    }
}
impl Ord for Route {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.heat_loss + other.mahatan).cmp(&(self.heat_loss + self.mahatan))
    }
}
impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        (other.heat_loss + other.mahatan).eq(&(self.heat_loss + self.mahatan))
    }
}
impl Eq for Route {}

#[derive(Debug)]
struct Puzzle {
    input: Vec<Vec<usize>>,
    heap: BinaryHeap<Route>,
    cache: HashMap<Coord, usize>,
    end: Coord,
    max_rows: usize,
    max_cols: usize,
}

impl Puzzle {
    fn new(input: &str, start: Coord, end: Coord) -> Self {
        let mut heap = BinaryHeap::new();
        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            heap.push(Route {
                heat_loss: 0,
                mahatan: start.mahattan(&end),
                nodes: vec![start],
                last_dir: dir,
            });
        }
        let cache = HashMap::from([(Coord(0,0), 0)]);
        Self {
            input: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as usize)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
            heap,
            cache,
            end,
            max_rows: input.lines().count(),
            max_cols: input.lines().next().unwrap().len(),
        }
    }
    fn num_at(&self, loc: Coord) -> usize {
        *self.input.get(loc.0).unwrap().get(loc.1).unwrap()
    }
    fn new_astar(&mut self, old: &Route) -> Vec<Route> {
        let mut ret = vec![];
        let old_row = old.nodes.last().unwrap().0;
        let old_col = old.nodes.last().unwrap().1;
        let mut addition_heat_loss = 0;
        let mut new_nodes = old.nodes.clone();

        match old.last_dir {
            Direction::Up => {
                for i in 1..=3 {
                    if let Some(row) = old_row.checked_sub(i) {
                        let new_coord = Coord(row, old_col);
                        if old.nodes.contains(&new_coord) {
                            break;
                        } else {
                            addition_heat_loss += self.num_at(new_coord);
                            let new_heat_loss = old.heat_loss + addition_heat_loss.clone();
                            new_nodes.push(new_coord);
                            if *self.cache.entry(new_coord).or_insert(usize::MAX) >= new_heat_loss {
                                *self.cache.get_mut(&new_coord).unwrap() = new_heat_loss;
                                for direction in [Direction::Left, Direction::Right] {
                                    ret.push(Route {
                                        heat_loss: new_heat_loss,
                                        mahatan: new_coord.mahattan(&self.end),
                                        nodes: new_nodes.clone(),
                                        last_dir: direction,
                                    });
                                }
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
            Direction::Down => {
                if old_row != self.max_rows - 1 {
                    let row_start = (old_row + 1).min(self.max_rows - 1);
                    let row_end = (old_row + 3).min(self.max_rows - 1);
                    for row in row_start..=row_end {
                        let new_coord = Coord(row, old_col);
                        if old.nodes.contains(&new_coord) {
                            break;
                        } else {
                            addition_heat_loss += self.num_at(new_coord);
                            let new_heat_loss = old.heat_loss + addition_heat_loss.clone();
                            new_nodes.push(new_coord);
                            if *self.cache.entry(new_coord).or_insert(usize::MAX) >= new_heat_loss {
                                *self.cache.get_mut(&new_coord).unwrap() = new_heat_loss;
                                for direction in [Direction::Left, Direction::Right] {
                                    ret.push(Route {
                                        heat_loss: new_heat_loss,
                                        mahatan: new_coord.mahattan(&self.end),
                                        nodes: new_nodes.clone(),
                                        last_dir: direction,
                                    });
                                }
                            }
                        }
                    }
                }
            }
            Direction::Left => {
                for i in 1..=3 {
                    if let Some(col) = old_col.checked_sub(i) {
                        let new_coord = Coord(old_row, col);
                        if old.nodes.contains(&new_coord) {
                            break;
                        } else {
                            addition_heat_loss += self.num_at(new_coord);
                            let new_heat_loss = old.heat_loss + addition_heat_loss.clone();
                            new_nodes.push(new_coord);
                            if *self.cache.entry(new_coord).or_insert(usize::MAX) >= new_heat_loss {
                                *self.cache.get_mut(&new_coord).unwrap() = new_heat_loss;
                                for direction in [Direction::Up, Direction::Down] {
                                    ret.push(Route {
                                        heat_loss: new_heat_loss,
                                        mahatan: new_coord.mahattan(&self.end),
                                        nodes: new_nodes.clone(),
                                        last_dir: direction,
                                    });
                                }
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
            Direction::Right => {
                if old_col != self.max_cols - 1 {
                    let col_start = (old_col + 1).min(self.max_cols - 1);
                    let col_end = (old_col + 3).min(self.max_cols - 1);
                    for col in col_start..=col_end {
                        let new_coord = Coord(old_row, col);
                        if old.nodes.contains(&new_coord) {
                            break;
                        } else {
                            addition_heat_loss += self.num_at(new_coord);
                            let new_heat_loss = old.heat_loss + addition_heat_loss.clone();
                            new_nodes.push(new_coord);
                            if *self.cache.entry(new_coord).or_insert(usize::MAX) >= new_heat_loss {
                                *self.cache.get_mut(&new_coord).unwrap() = new_heat_loss;
                                for direction in [Direction::Up, Direction::Down] {
                                    ret.push(Route {
                                        heat_loss: new_heat_loss,
                                        mahatan: new_coord.mahattan(&self.end),
                                        nodes: new_nodes.clone(),
                                        last_dir: direction,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        ret
    }

    fn run(&mut self) -> usize {
        let mut i = 0;
        while let Some(route) = self.heap.pop() {
            if route.nodes.last().unwrap() == &self.end {
                dbg!(&route);
                dbg!(&i);
                dbg!(&self.heap.len());
                return route.heat_loss as usize;
            } else {
                for new_astar in self.new_astar(&route) {
                    // dbg!(&new_astar);
                    self.heap.push(new_astar);
                }
            }
            i += 1;
            // if i == 1000 {
            //     dbg!(&self.heap.peek());
            //     dbg!(&self.heap.len());
            //     break;
            // }
        }

        panic!("cant find end coord")
    }
}

pub fn process(input: &str) -> usize {
    let max_rows = input.lines().count();
    let max_cols = input.lines().next().unwrap().len();
    Puzzle::new(input, Coord(0, 0), Coord(max_rows - 1, max_cols - 1)).run()
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
