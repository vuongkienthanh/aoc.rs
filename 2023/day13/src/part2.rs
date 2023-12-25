fn two_vec_char_diff(a: &[char], b: &[char]) -> Vec<usize> {
    a.iter()
        .zip(b.iter())
        .enumerate()
        .filter_map(|(i, (x, y))| if x != y { Some(i) } else { None })
        .collect::<Vec<_>>()
}
pub fn process(_input: &str) -> usize {
    todo!("part2")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
        assert_eq!(process(input), 400);
    }
}
