type Coord = (isize, isize);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn to_dir_range(input: &str) -> Vec<(Direction, isize)> {
    let mut ret = vec![];
    for line in input.lines() {
        let third = line.split_ascii_whitespace().skip(2).next().unwrap();
        let dir = match third.chars().nth(7).unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => panic!("only 0123"),
        };
        let range = usize::from_str_radix(third.get(2..7).unwrap(), 16).unwrap() as isize;
        ret.push((dir, range));
    }
    ret
}
fn to_coords(input: &[(Direction, isize)]) -> Vec<Coord> {
    let mut cur = (0, 0);
    let mut ret = vec![];
    for (dir, range) in input {
        match dir {
            Direction::Right => cur = (cur.0, cur.1 + range),
            Direction::Down => cur = (cur.0 + range, cur.1),
            Direction::Left => cur = (cur.0, cur.1 - range),
            Direction::Up => cur = (cur.0 - range, cur.1),
        };
        ret.push(cur);
    }
    ret
}
fn determinant(a: &Coord, b: &Coord) -> isize {
    a.0 * b.1 - a.1 * b.0
}
pub fn process(input: &str) -> isize {
    let dir_range = to_dir_range(input);
    let coords = to_coords(&dir_range);

    // pick theorem
    // i + b = A + b/2 + 1

    // A - Area is shoelace formula
    let first = coords.first().unwrap();
    let last = coords.last().unwrap();
    let mut shoelace = determinant(last, first);
    for pair in coords.windows(2) {
        let a = pair.get(0).unwrap();
        let b = pair.get(1).unwrap();
        shoelace += determinant(a, b);
    }
    shoelace = (shoelace / 2).abs();

    dbg!(shoelace);
    // i - interior

    // b - border
    let border = dir_range.iter().map(|(_,range)| *range).sum::<isize>();

    // shoelace + border/2 + 1
        0
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
