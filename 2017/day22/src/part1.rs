use aoc_helper::direction::Direction;
use fxhash::FxHashSet as Set;
type Point = (isize, isize);

pub fn process(_input: &str) -> usize {
    let mut ans = 0;
    let cols = _input.lines().count();
    let rows = _input.lines().next().unwrap().chars().count();
    let mut virus = ((rows as isize / 2, cols as isize / 2), Direction::Up);
    let mut map: Set<Point> =
        _input
            .lines()
            .enumerate()
            .fold(Set::default(), |mut acc, (row, line)| {
                for (col, c) in line.char_indices() {
                    if c == '#' {
                        acc.insert((row as isize, col as isize));
                    }
                }
                acc
            });
    for _ in 0..10_000 {
        // if infected
        let new_dir = if map.remove(&virus.0) {
            virus.1.turn_right()
        } else {
            ans += 1;
            map.insert(virus.0);
            virus.1.turn_left()
        };
        let new_loc = match new_dir {
            Direction::Up => (virus.0.0 - 1, virus.0.1),
            Direction::Down => (virus.0.0 + 1, virus.0.1),
            Direction::Left => (virus.0.0, virus.0.1 - 1),
            Direction::Right => (virus.0.0, virus.0.1 + 1),
        };
        virus = (new_loc, new_dir);
    }
    ans
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"..#
#..
..."#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 5587);
    }
}
