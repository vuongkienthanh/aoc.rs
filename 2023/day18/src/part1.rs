use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Border {
    Vertical,
    Horizontal,
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
}
type Coord = (isize, isize);

#[derive(Debug)]
struct Point {
    border: Border,
}

#[rustfmt::skip]
fn parse_input(input: &str) -> HashMap<Coord, Point> {
    let mut ret = HashMap::new();
    let first_direction = input.lines().next().unwrap().split_once(" ").unwrap().0;
    let mut line_iter = input.lines().peekable();
    let mut cur = (0isize, 0isize);
    for _ in 0..input.lines().count() {
        let mut line = line_iter.next().unwrap().split_ascii_whitespace();
        let direction = line.next().unwrap();
        let range = line.next().unwrap().parse::<isize>().unwrap();
        match direction {
            "R" => {
                let (cur_x, cur_y) = cur;
                for i in 1..range { ret.insert( (cur_x, cur_y + i), Point { border: Border::Horizontal },); }
                cur = (cur_x, cur_y + range);
                let next_dir = { if let Some(next_line) = line_iter.peek() { next_line.split_once(" ").unwrap().0 } else { first_direction } };
                match next_dir {
                    "U" => ret.insert( cur, Point { border: Border::BottomRight },),
                    "D" => ret.insert( cur, Point { border: Border::TopRight },),
                    _ => panic!("R -> U D"),
                };
            }
            "L" => {
                let (cur_x, cur_y) = cur;
                for i in 1..range { ret.insert( (cur_x, cur_y - i), Point { border: Border::Horizontal },); }
                cur = (cur_x, cur_y - range);
                let next_dir = { if let Some(next_line) = line_iter.peek() { next_line.split_once(" ").unwrap().0 } else { first_direction } };
                match next_dir {
                    "U" => ret.insert( cur, Point { border: Border::BottomLeft },),
                    "D" => ret.insert( cur, Point { border: Border::TopLeft },),
                    _ => panic!("L -> U D"),
                };
            }
            "U" => {
                let (cur_x, cur_y) = cur;
                for i in 1..range { ret.insert( (cur_x - i, cur_y), Point { border: Border::Vertical,  },); }
                cur = (cur_x - range, cur_y);
                let next_dir = { if let Some(next_line) = line_iter.peek() { next_line.split_once(" ").unwrap().0 } else { first_direction } };
                match next_dir {
                    "L" => ret.insert( cur, Point { border: Border::TopRight },),
                    "R" => ret.insert( cur, Point { border: Border::TopLeft },),
                    _ => panic!("U -> L R"),
                };
            }
            "D" => {
                let (cur_x, cur_y) = cur;
                for i in 1..range { ret.insert( (cur_x + i, cur_y), Point { border: Border::Vertical },); }
                cur = (cur_x + range, cur_y);
                let next_dir = { if let Some(next_line) = line_iter.peek() { next_line.split_once(" ").unwrap().0 } else { first_direction } };
                match next_dir {
                    "L" => ret.insert( cur, Point { border: Border::BottomRight },),
                    "R" => ret.insert( cur, Point { border: Border::BottomLeft },),
                    _ => panic!("U -> L R"),
                };
            }
            _ => panic!("no such direction"),
        }
    }
    ret
}

pub fn process(input: &str) -> usize {
    let borders = parse_input(input);
    let min_x = borders.keys().into_iter().map(|(x, _)| *x).min().unwrap();
    let max_x = borders.keys().into_iter().map(|(x, _)| *x).max().unwrap();
    let min_y = borders.keys().into_iter().map(|(_, y)| *y).min().unwrap();
    let max_y = borders.keys().into_iter().map(|(_, y)| *y).max().unwrap();

    let mut ret = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if borders.contains_key(&(x, y)) {
                ret += 1;
            } else {
                let left = (min_y..y)
                    .filter_map(|yi| borders.get(&(x, yi)))
                    .collect::<Vec<_>>();
                let bars = left.iter().filter(|p| p.border == Border::Vertical).count();
                let topleft = left.iter().filter(|p| p.border == Border::TopLeft).count();
                let topright = left.iter().filter(|p| p.border == Border::TopRight).count();
                let lines = bars + topleft.abs_diff(topright);
                if lines % 2 == 1 {
                    ret += 1;
                }
            }
        }
    }
    ret
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;
        assert_eq!(process(input), 62);
    }
}
