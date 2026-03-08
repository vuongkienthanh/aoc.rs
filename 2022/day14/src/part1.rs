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
                for cell in map
                    .get_mut(b)
                    .unwrap()
                    .iter_mut()
                    .take(right + 1)
                    .skip(left)
                {
                    *cell = '#';
                }
            }
            if a == x {
                let up = b.min(y);
                let down = b.max(y);
                for row in map.iter_mut().take(down + 1).skip(up) {
                    *row.get_mut(a - min_x + 1).unwrap() = '#';
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

