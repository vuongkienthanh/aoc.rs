use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let max_y = input.iter().flatten().map(|(_, y)| *y).max().unwrap();
    let mut map = vec![vec!['.'; (max_y + 1) * 2 + 1]; max_y + 2];
    for v in input {
        for w in v.windows(2) {
            let (a, b) = w[0];
            let (x, y) = w[1];
            if b == y {
                let left = (a + max_y - 500).min(x + max_y - 500) + 1;
                let right = (a + max_y - 500).max(x + max_y - 500) + 1;
                for i in left..=right {
                    map[b][i] = '#';
                }
            } else if a == x {
                let up = b.min(y);
                let down = b.max(y);
                for i in up..=down {
                    map[i][a + max_y + 1 - 500] = '#';
                }
            } else {
                panic!("AD");
            }
        }
    }
    map[0][max_y + 1] = 'o';
    let mut current = vec![(max_y + 1, 0)];
    for _ in 0..=max_y {
        let mut new = vec![];

        for sand in current {
            for (a, b) in [
                (sand.0, sand.1 + 1),
                (sand.0 - 1, sand.1 + 1),
                (sand.0 + 1, sand.1 + 1),
            ] {
                if map[b][a] == '.' {
                    map[b][a] = 'o';
                    new.push((a, b));
                }
            }
        }

        current = new;
    }
    map.into_iter().flatten().filter(|x| *x == 'o').count()
}