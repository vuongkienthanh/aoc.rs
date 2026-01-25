pub mod part1;
pub mod part2;

use intcode::{Computer, RunResult, parse};
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

fn build_map(input: &str) -> (BTreeSet<Point>, (Point, Direction)) {
    let mut camera = Computer::new(parse(input));
    let mut seen = BTreeSet::new();
    let (mut loc, mut robo, mut dir) = ((0, 0), (0, 0), Direction::Up);
    loop {
        let output = if let RunResult::Output(output) = camera.long_run() {
            output as u8 as char
        } else {
            break;
        };
        match output {
            '.' => {
                loc = (loc.0 + 1, loc.1);
            }
            '#' => {
                seen.insert(loc);
                loc = (loc.0 + 1, loc.1);
            }
            '^' => {
                dir = Direction::Up;
                robo = loc;
                seen.insert(loc);
                loc = (loc.0 + 1, loc.1);
            }
            'v' => {
                dir = Direction::Down;
                robo = loc;
                seen.insert(loc);
                loc = (loc.0 + 1, loc.1);
            }
            '<' => {
                dir = Direction::Left;
                robo = loc;
                seen.insert(loc);
                loc = (loc.0 + 1, loc.1);
            }
            '>' => {
                dir = Direction::Right;
                robo = loc;
                seen.insert(loc);
                loc = (loc.0 + 1, loc.1);
            }
            '\n' => {
                loc = (0, loc.1 + 1);
            }
            _ => panic!(),
        }
        print!("{}", output);
    }
    (seen, (robo, dir))
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

fn get_a(path: &[Turn]) -> Vec<Turn> {
    let mut a = (&path[..2]).to_vec();
    let mut i = 2;
    loop {
        i += 1;
        let next_a = (&path[..i]).to_vec();
        if path.windows(i).filter(|c| c == &&next_a).count() > 1 {
            a = next_a;
        } else {
            break;
        }
    }
    a
}

fn divide_a(path: Vec<Turn>, a: &[Turn]) -> Vec<Vec<Turn>> {
    let mut ans = vec![];
    let mut new = vec![];
    let mut i = 0;
    loop {
        if i > path.len() - a.len() {
            break;
        }
        if &path[i..i + a.len()] == a {
            i += a.len();
            if !new.is_empty() {
                ans.push(new);
                new = vec![];
            }
        } else {
            new.push(path[i].clone());
            i += 1;
        }
    }
    new.extend(path[i..].iter().cloned());

    if !new.is_empty() {
        ans.push(new);
    }
    ans
}
fn get_b_c(path: &[Vec<Turn>]) -> Vec<Turn> {
    let first_path = path.get(0).unwrap();
    let mut b = (first_path[..2]).to_vec();
    let mut i = 2;
    loop {
        i += 1;
        if i > first_path.len() {
            break;
        }
        let next_b = (first_path[..i]).to_vec();
        if path
            .iter()
            .map(|p| p.windows(i).filter(|c| c == &&next_b).count())
            .sum::<usize>()
            > 1
        {
            b = next_b;
        } else {
            break;
        }
    }
    b
}

fn divide_b_c(path: Vec<Vec<Turn>>, b: &[Turn]) -> Vec<Vec<Turn>> {
    let mut ans = vec![];
    for p in path {
        let mut i = 0;
        let mut new = vec![];
        loop {
            if i > p.len() - b.len() {
                break;
            }
            if &p[i..i + b.len()] == b {
                i += b.len();
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
        for (p, c) in [a, b, c].into_iter().zip(['A', 'B', 'C']) {
            if path[i..i + p.len()] == *p {
                ans.push(c);
                ans.push(',');
                i += p.len();
                break;
            }
        }
    }
    ans[..ans.len() - 1].into_iter().collect()
}

fn build_move_function(a: &[Turn]) -> String {
    let mut ans = vec![];
    for t in a {
        match t {
            Turn::Left(z) => ans.push(format!("L,{z}")),
            Turn::Right(z) => ans.push(format!("R,{z}")),
        }
    }
    ans.join(",")
}
