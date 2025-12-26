use crate::build_grid;
use aoc_helper::adj::grid::adj4;

pub fn process(_input: &str) -> usize {
    let data = build_grid(_input);

    let mut ans = 0;
    let mut seen = [[false; 128]; 128];
    for row in 0..128 {
        for col in 0..128 {
            if seen[row][col] || data[row][col] == 0 {
                continue;
            }

            ans += 1;

            let mut current: Vec<(usize, usize)> = vec![(row, col)];

            while !current.is_empty() {
                let mut new = vec![];

                for (row, col) in current {
                    if data[row][col] == 1 {
                        seen[row][col] = true;
                        new.extend(
                            adj4((row, col), 128, 128)
                                .into_iter()
                                .flatten()
                                .filter(|(row, col)| !seen[*row][*col]),
                        );
                    }
                }
                current = new;
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"flqrgnkx"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 1242);
    }
}
