pub mod part1;
pub mod part2;

use intcode::{Computer, RunResult};
use std::collections::BTreeSet;

type Point = (usize, usize);

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Turn {
    Left(usize),
    Right(usize),
}

fn build_map(mut camera: Computer) -> (BTreeSet<Point>, (Point, Direction)) {
    let mut seen = BTreeSet::new();
    let (mut loc, mut robo_loc, mut dir) = ((0, 0), (0, 0), Direction::Up);
    while let RunResult::Output(output) = camera.long_run() {
        match output as u8 {
            b'.' => {
                loc = (loc.0 + 1, loc.1);
            }
            b'#' => {
                seen.insert(loc);
                loc = (loc.0 + 1, loc.1);
            }
            b'^' => {
                dir = Direction::Up;
                robo_loc = loc;
                seen.insert(loc);
                loc = (loc.0 + 1, loc.1);
            }
            b'v' => {
                dir = Direction::Down;
                robo_loc = loc;
                seen.insert(loc);
                loc = (loc.0 + 1, loc.1);
            }
            b'<' => {
                dir = Direction::Left;
                robo_loc = loc;
                seen.insert(loc);
                loc = (loc.0 + 1, loc.1);
            }
            b'>' => {
                dir = Direction::Right;
                robo_loc = loc;
                seen.insert(loc);
                loc = (loc.0 + 1, loc.1);
            }
            b'\n' => {
                loc = (0, loc.1 + 1);
            }
            _ => panic!(),
        }
    }
    (seen, (robo_loc, dir))
}

fn get_path(map: &BTreeSet<Point>, mut robo: (Point, Direction)) -> Vec<Turn> {
    fn left(map: &BTreeSet<Point>, p: Point) -> Option<usize> {
        (0..p.0)
            .rev()
            .take_while(|x| map.contains(&(*x, p.1)))
            .last()
    }
    fn right(map: &BTreeSet<Point>, p: Point) -> Option<usize> {
        (p.0 + 1..).take_while(|x| map.contains(&(*x, p.1))).last()
    }
    fn up(map: &BTreeSet<Point>, p: Point) -> Option<usize> {
        (0..p.1)
            .rev()
            .take_while(|y| map.contains(&(p.0, *y)))
            .last()
    }
    fn down(map: &BTreeSet<Point>, p: Point) -> Option<usize> {
        (p.1 + 1..).take_while(|y| map.contains(&(p.0, *y))).last()
    }
    let mut ans = vec![];
    loop {
        match robo.1 {
            Direction::Up => {
                if let Some(x) = left(map, robo.0) {
                    ans.push(Turn::Left(robo.0.0 - x));
                    robo = ((x, robo.0.1), Direction::Left);
                } else if let Some(x) = right(map, robo.0) {
                    ans.push(Turn::Right(x - robo.0.0));
                    robo = ((x, robo.0.1), Direction::Right);
                } else {
                    break;
                }
            }
            Direction::Down => {
                if let Some(x) = left(map, robo.0) {
                    ans.push(Turn::Right(robo.0.0 - x));
                    robo = ((x, robo.0.1), Direction::Left);
                } else if let Some(x) = right(map, robo.0) {
                    ans.push(Turn::Left(x - robo.0.0));
                    robo = ((x, robo.0.1), Direction::Right);
                } else {
                    break;
                }
            }
            Direction::Left => {
                if let Some(y) = up(map, robo.0) {
                    ans.push(Turn::Right(robo.0.1 - y));
                    robo = ((robo.0.0, y), Direction::Up);
                } else if let Some(y) = down(map, robo.0) {
                    ans.push(Turn::Left(y - robo.0.1));
                    robo = ((robo.0.0, y), Direction::Down);
                } else {
                    break;
                }
            }
            Direction::Right => {
                if let Some(y) = up(map, robo.0) {
                    ans.push(Turn::Left(robo.0.1 - y));
                    robo = ((robo.0.0, y), Direction::Up);
                } else if let Some(y) = down(map, robo.0) {
                    ans.push(Turn::Right(y - robo.0.1));
                    robo = ((robo.0.0, y), Direction::Down);
                } else {
                    break;
                }
            }
        }
    }
    ans
}

// assuming first 2 turns is an move function
// expand it and check if it is still repeat elsewhere, replace it
fn get_a_b_c(paths: &[Vec<Turn>]) -> Vec<Turn> {
    let first_path = paths.first().unwrap();
    let mut v = (first_path[..2]).to_vec();
    let mut i = 2;
    loop {
        i += 1;
        if i > first_path.len() {
            break;
        }
        let next_v = (first_path[..i]).to_vec();
        if paths
            .iter()
            .map(|p| p.windows(i).filter(|c| c == &next_v).count())
            .sum::<usize>()
            > 1
        {
            v = next_v;
        } else {
            break;
        }
    }
    v
}

// trim out move function
fn divide_a_b_c(paths: Vec<Vec<Turn>>, v: &[Turn]) -> Vec<Vec<Turn>> {
    let mut ans = vec![];
    for p in paths {
        let mut i = 0;
        let mut new = vec![];
        loop {
            if i > p.len() - v.len() {
                break;
            }
            if &p[i..i + v.len()] == v {
                i += v.len();
                if !new.is_empty() {
                    ans.push(new);
                    new = vec![];
                }
            } else {
                new.push(p[i].clone());
                i += 1;
            }
        }
        new.extend(p[i..].iter().cloned());

        if !new.is_empty() {
            ans.push(new);
        }
    }
    ans
}

fn build_main_function(path: Vec<Turn>, a: &[Turn], b: &[Turn], c: &[Turn]) -> String {
    let mut ans = vec![];
    let mut i = 0;
    loop {
        if i >= path.len() {
            break;
        }
        for (p, c) in [a, b, c].into_iter().zip(["A", "B", "C"]) {
            if path[i..i + p.len()] == *p {
                ans.push(c);
                i += p.len();
                break;
            }
        }
    }
    ans.join(",")
}

fn build_move_function(v: &[Turn]) -> String {
    let mut ans = vec![];
    for t in v {
        match t {
            Turn::Left(x) => ans.push(format!("L,{x}")),
            Turn::Right(x) => ans.push(format!("R,{x}")),
        }
    }
    ans.join(",")
}
