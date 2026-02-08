use crate::{parse, CellType, Coord, CoordKey, Direction, IntersectionDirInfo};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Path {
    in_dir: Option<Direction>,
    coord: Coord,
    out_dir: Direction,
}

#[derive(Debug)]
struct CostIntersection {
    from_top: (usize, Vec<Path>),
    from_bottom: (usize, Vec<Path>),
    from_left: (usize, Vec<Path>),
    from_right: (usize, Vec<Path>),
}
impl Default for CostIntersection {
    fn default() -> Self {
        Self {
            from_top: (usize::MAX, Vec::new()),
            from_bottom: (usize::MAX, Vec::new()),
            from_left: (usize::MAX, Vec::new()),
            from_right: (usize::MAX, Vec::new()),
        }
    }
}
impl CostIntersection {
    fn get_dir(&self, dir: Direction) -> &(usize, Vec<Path>) {
        match dir {
            Direction::Up => &self.from_bottom,
            Direction::Down => &self.from_top,
            Direction::Left => &self.from_right,
            Direction::Right => &self.from_left,
        }
    }
    fn get_mut_dir(&mut self, dir: Direction) -> &mut (usize, Vec<Path>) {
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
    start_cost_intersection.from_top.0 = 0;
    start_cost_intersection.from_bottom.0 = 0;
    start_cost_intersection.from_left.0 = 0;
    start_cost_intersection.from_right.0 = 0;

    let mut stack = VecDeque::from([(
        0,
        Path {
            in_dir: None,
            coord: start,
            out_dir: Direction::Right,
        },
    )]);

    while let Some((
        cost,
        Path {
            in_dir,
            coord,
            out_dir,
        },
    )) = stack.pop_front()
    {
        for (expand_out_dir, expand_cost) in out_dir.expand() {
            if let Some(IntersectionDirInfo {
                dst: next_coord,
                cost: next_cost,
                dir_at_dst: next_dir,
                path_to_dst: _,
            }) = graph[&coord.into()].get_dir(expand_out_dir)
            {
                let next_cost = cost + expand_cost + next_cost;
                match next_cost.cmp(&costmap[&(*next_coord).into()].get_dir(*next_dir).0) {
                    std::cmp::Ordering::Less => {
                        let cost_intersection = costmap
                            .get_mut(&(*next_coord).into())
                            .unwrap()
                            .get_mut_dir(*next_dir);
                        cost_intersection.0 = next_cost;
                        cost_intersection.1 = vec![Path {
                            in_dir,
                            coord,
                            out_dir: expand_out_dir,
                        }];
                        stack.push_back((
                            next_cost,
                            Path {
                                in_dir: Some(*next_dir),
                                coord: *next_coord,
                                out_dir: *next_dir,
                            },
                        ));
                    }
                    std::cmp::Ordering::Equal => {
                        let cost_intersection = &mut costmap
                            .get_mut(&(*next_coord).into())
                            .unwrap()
                            .get_mut_dir(*next_dir)
                            .1;
                        let path = Path {
                            in_dir,
                            coord,
                            out_dir: expand_out_dir,
                        };
                        if !cost_intersection.contains(&path) {
                            cost_intersection.push(path);
                        }
                    }
                    std::cmp::Ordering::Greater => (),
                }
            }
        }
    }
    let mut stack = VecDeque::from_iter(
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Down,
        ]
        .into_iter()
        .min_by_key(|x| costmap[&end.into()].get_dir(*x).0)
        .map(|x| costmap[&end.into()].get_dir(x).1.as_slice())
        .unwrap()
        .iter(),
    );
    let mut tiles = HashSet::new();
    while let Some(Path {
        in_dir,
        coord,
        out_dir,
    }) = stack.pop_front()
    {
        let IntersectionDirInfo {
            dst,
            cost: _,
            dir_at_dst: _,
            path_to_dst,
        } = graph
            .get(&(*coord).into())
            .unwrap()
            .get_dir(*out_dir)
            .as_ref()
            .unwrap();
        tiles.insert(CoordKey::from(*dst));
        for tile in path_to_dst.as_slice() {
            tiles.insert(CoordKey::from(*tile));
        }
        if let Some(dir) = in_dir {
            for path in &costmap[&(*coord).into()].get_dir(*dir).1 {
                stack.push_back(path);
            }
        }
    }

    if cfg!(test) {
        for (k, v) in costmap.iter() {
            let paths = [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Down,
            ]
            .into_iter()
            .map(|x| &v.get_dir(x).1)
            .collect::<Vec<&Vec<Path>>>();
            for path in paths {
                if path.len() >= 2 {
                    println!("k: {k:?}, v: {v:?}");
                }
            }
        }
        for (i, line) in _grid.iter_rows().enumerate() {
            for (j, cell) in line.enumerate() {
                if tiles.contains(&(i, j).into()) {
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
    }

    tiles.len() + 1
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
