pub mod part1;
pub mod part2;

const DIRECTIONS: [(usize, char); 4] = [(0, 'U'), (1, 'D'), (2, 'L'), (3, 'R')];

pub fn md5hash(input: &str, history: &str) -> String {
    format!("{:x}", md5::compute(format!("{input}{history}")))
}
