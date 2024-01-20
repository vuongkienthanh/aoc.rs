use crate::{getxy, Grid, Point};
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Coord(usize, usize);
impl Coord {
    fn to_point(&self) -> Point {
        (self.0, self.1)
    }
    fn next_coords(&self, prev: Option<Coord>, grid: &Grid) -> Vec<Coord> {
        let mut ret = vec![];

        match getxy(grid, self.0, self.1) {
            '>' => ret.push(Coord(self.0, self.1 + 1)),
            '<' => ret.push(Coord(self.0, self.1 - 1)),
            '^' => ret.push(Coord(self.0 - 1, self.1)),
            'v' => ret.push(Coord(self.0 + 1, self.1)),
            '.' => {
                let mut up = None;
                let mut down = None;
                let mut left = None;
                let mut right = None;
                if self.0 == 0 {
                    down = Some(Coord(self.0 + 1, self.1));
                    right = Some(Coord(self.0, self.1 + 1));
                } else if self.0 == grid.len() - 1 {
                    up = Some(Coord(self.0 - 1, self.1));
                    right = Some(Coord(self.0, self.1 + 1));
                } else if self.1 == 0 {
                    down = Some(Coord(self.0 + 1, self.1));
                    right = Some(Coord(self.0, self.1 + 1));
                } else if self.1 == grid.first().unwrap().len() - 1 {
                    down = Some(Coord(self.0 + 1, self.1));
                    left = Some(Coord(self.0, self.1 - 1));
                } else {
                    up = Some(Coord(self.0 - 1, self.1));
                    down = Some(Coord(self.0 + 1, self.1));
                    left = Some(Coord(self.0, self.1 - 1));
                    right = Some(Coord(self.0, self.1 + 1));
                }
                for (opt_c, symbol) in [up, down, left, right].into_iter().zip("v^><".chars()) {
                    if let Some(c) = opt_c {
                        if getxy(grid, c.0, c.1) != symbol {
                            ret.push(c)
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
        if let Some(p) = prev {
            ret = ret.into_iter().filter(|c| c != &p).collect()
        };
        ret.into_iter()
            .filter(|c| getxy(grid, c.0, c.1) != '#')
            .collect()
    }
}

// len to branches, branches
type PathInfo = (usize, Vec<Point>);

fn parse_input(input: &str) -> HashMap<Point, PathInfo> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let maxx = grid.len() - 1;
    let maxy = grid.first().unwrap().len() - 1;
    let mut ret = HashMap::new();

    let mut nodes = VecDeque::from([(Coord(0, 1), None, Coord(0, 1), 0usize)]);

    while let Some((src, prev_node, cur_node, len)) = nodes.pop_front() {
        let next_coords = cur_node.next_coords(prev_node, &grid);
        if next_coords.is_empty() {
            // end
            if cur_node == Coord(maxx, maxy - 1) {
                ret.insert(src.to_point(), (len, vec![]));
            }
        } else if next_coords.len() > 1 {
            // branches
            if ret
                .insert(
                    src.to_point(),
                    (len + 1, next_coords.iter().map(|c| c.to_point()).collect()),
                )
                .is_none()
            {
                for node in next_coords {
                    nodes.push_back((node.clone(), Some(cur_node.clone()), node, 0));
                }
            }
        } else {
            //just continue node
            let node = next_coords.into_iter().next().unwrap();
            nodes.push_back((src, Some(cur_node), node, len + 1));
        }
    }

    ret
}

pub fn process(_input: &str) -> usize {
    let parsed = parse_input(_input);
    let mut ret: Vec<usize> = vec![];
    let start = (0, 1);
    let first_state = parsed.get(&start).unwrap().clone();
    let mut stack = VecDeque::from([first_state]);
    while let Some(state) = stack.pop_front() {
        if state.1.is_empty() {
            ret.push(state.0);
        } else {
            for branch in state.1 {
                let (new_len, new_branches) = parsed.get(&branch).unwrap().clone();
                stack.push_back((state.0 + new_len, new_branches));
            }
        }
    }
    ret.into_iter().max().unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;
        assert_eq!(process(input), 94);
    }
}
