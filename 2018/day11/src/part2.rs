use crate::power_level;

pub fn process(_input: &str) -> String {
    let grid_serial_number: usize = _input.parse().unwrap();
    let (x, y, size) = find(grid_serial_number);
    format!("{x},{y},{size}")
}
fn find(grid_serial_number: usize) -> (usize, usize, usize) {
    let (mut bx, mut by, mut bs, mut best) = (0, 0, 0, 0);
    let mut sum = [[0; 301]; 301];
    for y in 1..=300 {
        for x in 1..=300 {
            let p = power_level(grid_serial_number, (x, y));
            sum[y][x] = p + sum[y - 1][x] + sum[y][x - 1] - sum[y - 1][x - 1];
        }
    }
    for s in 1..=300 {
        for y in s..=300 {
            for x in s..=300 {
                let total = sum[y][x] - sum[y - s][x] - sum[y][x - s] + sum[y - s][x - s];
                if total > best {
                    (bx, by, bs, best) = (x, y, s, total);
                }
            }
        }
    }

    (bx - bs + 1, by - bs + 1, bs)
}
