use crate::{parse_to_graph, parse_to_grid, print_graph, print_grid, CoordKey, Direction};
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
    let (grid, start, end) = parse_to_grid(_input);
    let graph = parse_to_graph(&grid, start, end);

    // print_grid(&grid, &graph);
    // print_graph(&graph);

    let mut costmap = HashMap::new();
    for node in graph.keys() {
        costmap.insert(*node, CostIntersection::default());
    }
    let start_key = CoordKey::from(start);
    let start_node = costmap.get_mut(&start_key).unwrap();
    start_node.from_top = 0;
    start_node.from_bottom = 0;
    start_node.from_left = 0;
    start_node.from_right = 0;

    let mut stack = VecDeque::from([(start_key, Direction::Right, 0)]);
    while let Some((pos, dir, cost)) = stack.pop_front() {
        for (expand_dir, expand_cost) in dir.expand() {
            if let Some((next_pos, next_dir, next_cost)) = graph[&pos].get_dir(expand_dir) {
                let next_cost = cost + expand_cost + next_cost;
                let next_pos_key = CoordKey::from(*next_pos);
                if next_cost < *costmap[&next_pos_key].get_dir(*next_dir) {
                    stack.push_back((next_pos_key, *next_dir, next_cost));
                    *costmap
                        .get_mut(&next_pos_key)
                        .unwrap()
                        .get_mut_dir(*next_dir) = next_cost;
                }
            }
        }
    }
    let end_node = &costmap[&CoordKey::from(end)];
    [
        end_node.from_top,
        end_node.from_bottom,
        end_node.from_left,
        end_node.from_right,
    ]
    .into_iter()
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
