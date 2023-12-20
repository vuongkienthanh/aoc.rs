use itertools::Itertools;

fn expand_input(input: &str) -> Vec<Vec<char>> {
    let row_count = input.lines().count();
    let col_count = input.lines().next().unwrap().len();

    let expand_row_indices = (0..row_count)
        .filter_map(|i| {
            if input.lines().nth(i).unwrap().chars().all(|c| c == '.') {
                Some(i)
            } else {
                None
            }
        })
        .rev()
        .collect::<Vec<usize>>();

    let expand_col_indices = (0..col_count)
        .filter_map(|j| {
            if input
                .lines()
                .map(|line| line.chars().nth(j).unwrap())
                .all(|c| c == '.')
            {
                Some(j)
            } else {
                None
            }
        })
        .rev()
        .collect::<Vec<usize>>();

    let mut expanded = vec![];

    for i in 0..row_count {
        let mut line = input.lines().nth(i).unwrap().chars().collect::<Vec<char>>();
        for idx in &expand_col_indices {
            line.insert(*idx, '.');
        }
        expanded.push(line);
    }

    let insert_row = (0..(col_count + expand_col_indices.len()))
        .into_iter()
        .map(|_| '.')
        .collect::<Vec<char>>();

    for i in expand_row_indices {
        expanded.insert(i, insert_row.clone());
    }
    expanded
}
pub fn process(_input: &str) -> usize {
    let expanded = expand_input(_input);

    let row_count = expanded.len();
    let col_count = expanded.get(0).unwrap().len();

    let galaxies = (0..row_count)
        .into_iter()
        .flat_map(|i| (0..col_count).into_iter().map(move |j| (i, j)))
        .filter(|(i, j)| expanded.get(*i).unwrap().get(*j).unwrap() == &'#')
        .collect::<Vec<(usize, usize)>>();

    galaxies
        .into_iter()
        .combinations(2)
        .map(|pair| {
            let p0 = pair.get(0).unwrap();
            let p1 = pair.get(1).unwrap();

            p1.0.abs_diff(p0.0) + p1.1.abs_diff(p0.1)
        })
        .sum()
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
