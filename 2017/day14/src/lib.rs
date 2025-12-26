pub mod part1;
pub mod part2;
use day10::{process_input, process_output, round64};

fn build_grid(input: &str) -> [[usize; 128]; 128] {
    let mut data = [[0usize; 128]; 128];

    for (row, item) in data.iter_mut().enumerate() {
        let v = process_input(format!("{input}-{row}").as_str());
        let l = round64(v);
        let hash = process_output(l);

        for (c, col) in hash.chars().zip((0..).step_by(4)) {
            let mut n = c.to_digit(16).unwrap() as usize;
            for i in (col..col + 4).rev() {
                item[i] = n & 1;
                n >>= 1;
            }
        }
    }
    data
}
