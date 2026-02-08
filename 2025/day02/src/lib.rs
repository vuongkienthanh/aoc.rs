pub mod parsing;
pub mod part1;
pub mod part2;
pub mod part2_s;

pub fn multi(n: usize, part_len: u32, times: u32) -> usize {
    (0..times).fold(0, |acc, _| acc * 10usize.pow(part_len) + n)
}
