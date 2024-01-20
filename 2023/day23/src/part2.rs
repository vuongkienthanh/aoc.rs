use crate::{getxy, Grid, Point};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Coord(usize, usize);
impl Coord {
    fn to_point(&self) -> Point {
        (self.0, self.1)
    }
    fn next_coords(&self, prev: Option<Coord>, grid: &Grid) -> Vec<Coord> {
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

        let mut ret = [up, down, left, right]
            .into_iter()
            .filter_map(|c| c)
            .filter(|c| getxy(grid, c.0, c.1) != '#')
            .collect::<Vec<_>>();
        if let Some(p) = prev {
            ret = ret.into_iter().filter(|c| c != &p).collect()
        };

        ret
    }
}

// branches ( len, target )
type PathInfo = HashSet<(usize, Point)>;

fn parse_input(input: &str) -> (HashMap<Point, PathInfo>, usize, usize) {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if "<>^v".contains(c) { '.' } else { c })
                .collect::<Vec<_>>()
        })
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
                ret.insert(src.to_point(), HashSet::from([(len, cur_node.to_point())]));
            }
        } else if next_coords.len() > 1 {
            // branches
            if ret
                .entry(src.to_point())
                .or_default()
                .insert((len, cur_node.to_point()))
            {
                for node in next_coords {
                    nodes.push_back((cur_node.clone(), Some(cur_node.clone()), node.clone(), 1));
                }
            }
        } else {
            //just continue node
            let node = next_coords.into_iter().next().unwrap();
            nodes.push_back((src, Some(cur_node), node, len + 1));
        }
    }

    (ret, maxx, maxy)
}

pub fn process(_input: &str) -> usize {
    let (parsed, maxx, maxy) = parse_input(_input);
    let mut ret: Vec<usize> = vec![];

    // for (k, v) in &parsed {
    //     for b in v {
    //         println!("({} {}) --{}--> ({} {}) ", k.0, k.1, b.0, b.1 .0, b.1 .1);
    //     }
    // }
    // println!(" num of nodes {}", parsed.len() + 1);

    let start = (0, 1);
    let start_branches = parsed.get(&start).unwrap().clone();
    let seen = vec![start];
    let mut stack = VecDeque::new();
    for (len, branch) in start_branches {
        stack.push_back((len, branch, seen.clone()));
    }

    while let Some((len, branch, seen)) = stack.pop_front() {
        if branch == (maxx, maxy - 1) {
            ret.push(len);
        } else if !seen.contains(&branch) {
            let mut new_seen = seen.clone();
            new_seen.push(branch);
            for (new_len, new_branch) in parsed.get(&branch).unwrap() {
                stack.push_front((len + *new_len, *new_branch, new_seen.clone()));
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
        assert_eq!(process(input), 154);
    }
}
