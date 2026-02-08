pub mod part1;
pub mod part2;

fn power_level(grid_serial_number: usize, (x, y): (usize, usize)) -> isize {
    let rack_id = 10 + x;
    let mut lvl = rack_id * y;
    lvl += grid_serial_number;
    lvl *= rack_id;
    lvl /= 100;
    lvl %= 10;
    lvl as isize - 5
}

fn build_grid(grid_serial_number: usize) -> [[isize; 300]; 300] {
    let mut grid = [[0; 300]; 300];

    for y in 0..300 {
        for x in 0..300 {
            grid[y][x] = power_level(grid_serial_number, (x + 1, y + 1));
        }
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(8, (3,5), 4)]
    #[case(57, (122,79), -5)]
    #[case(39, (217,196), 0)]
    #[case(71, (101,153), 4)]
    fn test_power_level(
        #[case] grid_serial_number: usize,
        #[case] (x, y): (usize, usize),
        #[case] expected: isize,
    ) {
        assert_eq!(power_level(grid_serial_number, (x, y)), expected);
    }
}
