use crate::GalaxyMap;

pub fn process(_input: &str) -> usize {
    let mut galaxies = GalaxyMap::new(_input);
    galaxies.solve(1_000_000)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_10_times() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
        let mut galaxies = GalaxyMap::new(input);
        assert_eq!(galaxies.solve(10), 1030);
    }

    #[test]
    fn test_process_100_times() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
        let mut galaxies = GalaxyMap::new(input);
        assert_eq!(galaxies.solve(100), 8410);
    }
}
