pub fn process(_input: &str) -> usize {
    let mut platform = _input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let rows = platform.len();
    let cols = platform.first().unwrap().len();

    for row in 1..rows {
        for col in 0..cols {
            let cell = platform.get(row).unwrap().get(col).unwrap();
            if cell == &'O' {
                let mut i = row - 1;
                let nearest_unmovable = loop {
                    if platform.get(i).unwrap().get(col).unwrap() != &'.' {
                        break Some(i);
                    } else if i == 0 {
                        break None;
                    } else {
                        i -= 1
                    }
                };

                //begin move
                *platform.get_mut(row).unwrap().get_mut(col).unwrap() = '.';
                if let Some(r) = nearest_unmovable {
                    *platform.get_mut(r + 1).unwrap().get_mut(col).unwrap() = 'O';
                } else {
                    *platform.get_mut(0).unwrap().get_mut(col).unwrap() = 'O';
                }
            }
        }
    }

    (0..rows)
        .map(|i| {
            let score = rows - i;
            platform
                .get(i)
                .unwrap()
                .into_iter()
                .filter(|c| c == &&'O')
                .count()
                * score
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
        assert_eq!(process(input), 136);
    }
}
