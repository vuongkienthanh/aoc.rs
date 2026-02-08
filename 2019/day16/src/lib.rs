pub mod part1;
pub mod part2;

pub fn parse(input: &str) -> Vec<isize> {
    input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as isize)
        .collect()
}

pub fn parse_u8(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect()
}

