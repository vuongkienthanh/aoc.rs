use crate::GalaxyMap;

pub fn process(_input: &str) -> usize {
    let mut galaxies = GalaxyMap::new(_input);
    galaxies.solve(2)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
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
        assert_eq!(process(input), 374);
    }
}
