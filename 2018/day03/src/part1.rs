use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut grid = vec![vec![0; 1000]; 1000];
    for (_, row, col, wide, tall) in input {
        for i in row..row + tall {
            for j in col..col + wide {
                grid[i][j] += 1;
            }
        }
    }
    grid.into_iter()
        .flat_map(|line| line.into_iter())
        .filter(|x| *x > 1)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 4);
    }
}
