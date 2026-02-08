use super::{directions, parse};

pub fn process(_input: &str) -> usize {
    let grid = parse(_input);
    (1..10)
        .fold(
            grid.indexed_iter()
                .filter(|(_, c)| **c == 0)
                .map(|(x, _)| ::std::collections::HashSet::from([[x.0, x.1]]))
                .collect::<Vec<_>>(),
            |paths, i| {
                paths
                    .into_iter()
                    .map(|path| {
                        path.into_iter()
                            .flat_map(|x| {
                                directions(x, &grid)
                                    .into_iter()
                                    .filter(|new_x| grid[(*new_x).into()] == i)
                            })
                            .collect()
                    })
                    .collect()
            },
        )
        .into_iter()
        .map(|x| x.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        assert_eq!(process(input), 36);
    }
}
