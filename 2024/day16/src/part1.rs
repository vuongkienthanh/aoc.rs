use crate::{parse, Direction, IntersectionDirInfo};
use std::collections::{HashMap, VecDeque};

struct CostIntersection {
    from_top: usize,
    from_bottom: usize,
    from_left: usize,
    from_right: usize,
}
impl Default for CostIntersection {
    fn default() -> Self {
        Self {
            from_top: usize::MAX,
            from_bottom: usize::MAX,
            from_left: usize::MAX,
            from_right: usize::MAX,
        }
    }
}
impl CostIntersection {
    fn get_dir(&self, dir: Direction) -> &usize {
        match dir {
            Direction::Up => &self.from_bottom,
            Direction::Down => &self.from_top,
            Direction::Left => &self.from_right,
            Direction::Right => &self.from_left,
        }
    }
    fn get_mut_dir(&mut self, dir: Direction) -> &mut usize {
        match dir {
            Direction::Up => &mut self.from_bottom,
            Direction::Down => &mut self.from_top,
            Direction::Left => &mut self.from_right,
            Direction::Right => &mut self.from_left,
        }
    }
}

pub fn process(_input: &str) -> usize {
    let (start, end, _grid, graph) = parse(_input);

    let mut costmap = HashMap::new();
    for node in graph.keys() {
        costmap.insert(*node, CostIntersection::default());
    }
    let start_cost_intersection = costmap.get_mut(&start.into()).unwrap();
    start_cost_intersection.from_top = 0;
    start_cost_intersection.from_bottom = 0;
    start_cost_intersection.from_left = 0;
    start_cost_intersection.from_right = 0;

    let mut stack = VecDeque::from([(start, 0, Direction::Right)]);
    while let Some((coord, cost, dir)) = stack.pop_front() {
        for (expand_dir, expand_cost) in dir.expand() {
            if let Some(IntersectionDirInfo {
                dst: next_coord,
                cost: next_cost,
                dir_at_dst: next_dir,
                path_to_dst: _,
            }) = graph[&coord.into()].get_dir(expand_dir)
            {
                let next_cost = cost + expand_cost + next_cost;
                if next_cost < *costmap[&(*next_coord).into()].get_dir(*next_dir) {
                    *costmap
                        .get_mut(&(*next_coord).into())
                        .unwrap()
                        .get_mut_dir(*next_dir) = next_cost;
                    stack.push_back((*next_coord, next_cost, *next_dir));
                }
            }
        }
    }
    [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .into_iter()
    .map(|x| *costmap[&end.into()].get_dir(x))
    .min()
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
        assert_eq!(process(input), 7036);
    }
}
