use fxhash::FxHashSet;

pub fn process(_input: &str) -> usize {
    let input: Vec<Vec<u8>> = _input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
    let mut seen = FxHashSet::default();
    let rows = input.len();
    let cols = input[0].len();
    for c in 0..cols {
        seen.insert((0, c));
        seen.insert((rows - 1, c));
    }
    for r in 1..rows - 1 {
        seen.insert((r, 0));
        seen.insert((r, cols - 1));
    }

    for r in 1..rows - 1 {
        let row = input.get(r).unwrap();
        let mut left_max = row[0];
        for (c, cell) in row.iter().enumerate() {
            if *cell > left_max {
                seen.insert((r, c));
                left_max = *cell;
                if left_max == 9 {
                    break;
                }
            }
        }
        let mut right_max = row.last().cloned().unwrap();
        for (c, cell) in row.iter().enumerate().rev() {
            if *cell > right_max {
                seen.insert((r, c));
                right_max = *cell;
                if right_max == 9 {
                    break;
                }
            }
        }
    }
    for c in 1..cols - 1 {
        let col: Vec<_> = input.iter().map(|r| r.get(c).unwrap()).cloned().collect();
        let mut top_max = col[0];
        for (r, cell) in col.iter().enumerate() {
            if *cell > top_max {
                seen.insert((r, c));
                top_max = *cell;
                if top_max == 9 {
                    break;
                }
            }
        }
        let mut bottom_max = col.last().cloned().unwrap();
        for (r, cell) in col.iter().enumerate().rev() {
            if *cell > bottom_max {
                seen.insert((r, c));
                bottom_max = *cell;
                if bottom_max == 9 {
                    break;
                }
            }
        }
    }

    seen.len()
}