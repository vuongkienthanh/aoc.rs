type Coord = (isize, isize);

fn parse_input(input: &str) -> Vec<Coord> {
    let mut cur = (0, 0);
    let mut ret = vec![];
    for line in input.lines() {
        let third = line.split_ascii_whitespace().skip(2).next().unwrap();
        let range = usize::from_str_radix(third.get(2..7).unwrap(), 16).unwrap() as isize;
        match third.chars().nth(7).unwrap() {
            '0' => cur = (cur.0, cur.1 + range),
            '1' => cur = (cur.0 + range, cur.1),
            '2' => cur = (cur.0, cur.1 - range),
            '3' => cur = (cur.0 - range, cur.1),
            _ => panic!("only 0123"),
        };
        ret.push(cur);
    }
    ret
}
fn cross_minus(a: &Coord, b: &Coord) -> isize {
    a.0 * b.1 - a.1*b.0
}
pub fn process(input: &str) -> usize {
    let parsed = parse_input(input);
    parsed.windows(2).map(|coords| {
        let a = coords.get(0).unwrap();
        let b = coords.get(1).unwrap();
    }
        );
    dbg!(parsed);
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
