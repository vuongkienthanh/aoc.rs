use crate::parsing::parse_input;
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let max_y = input.iter().flatten().map(|(_, y)| *y).max().unwrap();
    let mut map = Grid::from(vec![vec![0u8; (max_y + 1) * 2 + 1]; max_y + 2]);
    for v in input {
        for w in v.windows(2) {
            let (a, b) = w[0];
            let (x, y) = w[1];
            if b == y {
                let left = (a + max_y - 500).min(x + max_y - 500) + 1;
                let right = (a + max_y - 500).max(x + max_y - 500) + 1;
                map.iter_row_mut(y)
                    .take(right + 1)
                    .skip(left)
                    .for_each(|x| *x = 0b10u8);
            }
            if a == x {
                let up = b.min(y);
                let down = b.max(y);
                map.iter_col_mut(x + max_y + 1 - 500)
                    .take(down + 1)
                    .skip(up)
                    .for_each(|y| *y = 0b10u8);
            }
        }
    }
    let center = max_y + 1;
    map[(0, center)] = 1;

    for row in 1..=max_y + 1 {
        let top_row: Vec<_> = map
            .iter_row(row - 1)
            .take(center + row)
            .skip(center - row + 1)
            .cloned()
            .collect();
        for (bot, top) in map
            .iter_row_mut(row)
            .take(center + row)
            .skip(center - row + 1)
            .zip(&top_row)
        {
            if *top == 1 {
                *bot |= 1;
            }
        }
        for (bot, top) in map
            .iter_row_mut(row)
            .take(center + row - 1)
            .skip(center - row)
            .zip(&top_row)
        {
            if *top == 1 {
                *bot |= 1;
            }
        }
        for (bot, top) in map
            .iter_row_mut(row)
            .take(center + row + 1)
            .skip(center - row + 2)
            .zip(&top_row)
        {
            if *top == 1 {
                *bot |= 1;
            }
        }
    }
    map.into_iter().filter(|x| *x == 1).count()
}
