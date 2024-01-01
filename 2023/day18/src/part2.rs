use std::usize;
#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

fn parse_input(input: &str) -> Vec<(Direction, isize)> {
    input
        .lines()
        .map(|line| {
            let third = line.split_ascii_whitespace().skip(2).next().unwrap();
            let dir = match third.chars().nth(7).unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => panic!("only 0123"),
            };
            let range = usize::from_str_radix(third.get(2..7).unwrap(), 16).unwrap() as isize;
            (dir, range)
        })
        .collect::<Vec<_>>()
}
pub fn process(input: &str) -> usize {
    let parsed = parse_input(input);
    let mut cur: (isize, isize) = (0, 0);
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (0isize, 0isize, 0isize, 0isize);
    for (dir, range) in parsed.iter() {
        match dir {
            Direction::Right => cur = (cur.0, cur.1 + *range),
            Direction::Left => cur = (cur.0, cur.1 - *range),
            Direction::Up => cur = (cur.0 - *range, cur.1),
            Direction::Down => cur = (cur.0 + *range, cur.1),
        }
        min_x = min_x.min(cur.0);
        max_x = max_x.min(cur.0);
        min_y = min_y.min(cur.1);
        max_y = max_y.min(cur.1);
    }
    todo!("part2")
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
        assert_eq!(process(input), 952408144115);
    }
}
