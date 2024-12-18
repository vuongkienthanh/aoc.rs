use crate::{parse, CellType, CoordKey, Direction};
use std::collections::{HashMap, HashSet, VecDeque};
#[derive(Debug)]
struct CostIntersection {
    from_top: (usize, HashSet<CoordKey>),
    from_bottom: (usize, HashSet<CoordKey>),
    from_left: (usize, HashSet<CoordKey>),
    from_right: (usize, HashSet<CoordKey>),
}
impl Default for CostIntersection {
    fn default() -> Self {
        Self {
            from_top: (usize::MAX, HashSet::new()),
            from_bottom: (usize::MAX, HashSet::new()),
            from_left: (usize::MAX, HashSet::new()),
            from_right: (usize::MAX, HashSet::new()),
        }
    }
}
impl CostIntersection {
    fn get_dir(&self, dir: Direction) -> &(usize, HashSet<CoordKey>) {
        match dir {
            Direction::Up => &self.from_bottom,
            Direction::Down => &self.from_top,
            Direction::Left => &self.from_right,
            Direction::Right => &self.from_left,
        }
    }
    fn get_mut_dir(&mut self, dir: Direction) -> &mut (usize, HashSet<CoordKey>) {
        match dir {
            Direction::Up => &mut self.from_bottom,
            Direction::Down => &mut self.from_top,
            Direction::Left => &mut self.from_right,
            Direction::Right => &mut self.from_left,
        }
    }
}
pub fn process(_input: &str) -> usize {
    let (start, end, grid, graph) = parse(_input);

    // print_grid(&grid);
    // print_graph(&graph);
    // todo!("ASD")

    let mut costmap = HashMap::new();
    for node in graph.keys() {
        costmap.insert(*node, CostIntersection::default());
    }
    let start_cost_intersection = costmap.get_mut(&start.into()).unwrap();
    start_cost_intersection.from_top.0 = 0;
    start_cost_intersection.from_bottom.0 = 0;
    start_cost_intersection.from_left.0 = 0;
    start_cost_intersection.from_right.0 = 0;

    let mut stack = VecDeque::from([(start, 0, Direction::Right, HashSet::<CoordKey>::new())]);
    while let Some((coord, cost, dir, tiles)) = stack.pop_front() {
        for (expand_dir, expand_cost) in dir.expand() {
            if let Some((next_coord, next_cost, next_dir, path)) =
                graph[&coord.into()].get_dir(expand_dir)
            {
                let next_cost = cost + expand_cost + next_cost;
                match next_cost.cmp(&costmap[&(*next_coord).into()].get_dir(*next_dir).0) {
                    std::cmp::Ordering::Less => {
                        let mut next_tiles = tiles.clone();
                        next_tiles.insert(coord.into());
                        for tile in path {
                            next_tiles.insert((*tile).into());
                        }
                        let cost_intersection = costmap
                            .get_mut(&(*next_coord).into())
                            .unwrap()
                            .get_mut_dir(*next_dir);
                        cost_intersection.0 = next_cost;
                        cost_intersection.1 = next_tiles.clone();
                        stack.push_back((*next_coord, next_cost, *next_dir, next_tiles));
                    }
                    std::cmp::Ordering::Equal => {
                        let mut next_tiles = tiles.clone();
                        next_tiles.insert(coord.into());
                        for tile in path {
                            next_tiles.insert((*tile).into());
                        }
                        let cost_intersection = costmap
                            .get_mut(&(*next_coord).into())
                            .unwrap()
                            .get_mut_dir(*next_dir);
                        cost_intersection.1.extend(next_tiles.clone());
                        stack.push_back((*next_coord, next_cost, *next_dir, next_tiles));
                    }
                    std::cmp::Ordering::Greater => (),
                }
            }
        }
    }
    for (i, line) in grid.iter_rows().enumerate() {
        for (j, cell) in line.enumerate() {
            if costmap[&end.into()].from_bottom.1.contains(&(i, j).into()) {
                print!("O")
            } else {
                match cell {
                    CellType::Wall => print!("#"),
                    CellType::Empty => print!("."),
                    CellType::Intersection => print!("X"),
                }
            }
        }
        println!();
    }
    [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Down,
    ]
    .into_iter()
    .min_by_key(|x| costmap[&end.into()].get_dir(*x).0)
    .map(|x| costmap[&end.into()].get_dir(x).1.len() + 1)
    .unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
        assert_eq!(process(input), 45);
    }
}
