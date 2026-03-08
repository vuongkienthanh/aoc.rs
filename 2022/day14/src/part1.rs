use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    for (x, y) in input.iter().flatten() {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }
    let mut map = vec![vec!['.'; max_x - min_x + 3]; max_y + 1];
    for v in input {
        for w in v.windows(2) {
            let (a, b) = w[0];
            let (x, y) = w[1];
            if b == y {
                let left = (a - min_x).min(x - min_x) + 1;
                let right = (a - min_x).max(x - min_x) + 1;
                for i in left..=right {
                    map[b][i] = '#';
                }
            }
            if a == x {
                let up = b.min(y);
                let down = b.max(y);
                for i in up..=down {
                    map[i][a - min_x + 1] = '#';
                }
            }
        }
    }
    let start = (500 - min_x + 1, 0);
    'ans: loop {
        let mut sand = start;
        loop {
            if sand.1 == max_y {
                break 'ans;
            }
            if let Some((a, b)) = [
                (sand.0, sand.1 + 1),
                (sand.0 - 1, sand.1 + 1),
                (sand.0 + 1, sand.1 + 1),
            ]
            .into_iter()
            .find(|(a, b)| map[*b][*a] == '.')
            {
                sand = (a, b);
            } else {
                map[sand.1][sand.0] = 'o';
                break;
            }
        }
    }
    map.into_iter().flatten().filter(|x| *x == 'o').count()
}