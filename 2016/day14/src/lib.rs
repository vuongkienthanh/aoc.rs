pub mod part1;
pub mod part2;

pub fn find_three(input: &str) -> Option<char> {
    input
        .as_bytes()
        .windows(3)
        .find(|x| x[0] == x[1] && x[0] == x[2])
        .map(|x| x[0] as char)
}
pub fn find_five(input: &str) -> Vec<char> {
    input
        .as_bytes()
        .windows(5)
        .filter(|x| x[0] == x[1] && x[0] == x[2] && x[0] == x[3] && x[0] == x[4])
        .map(|x| x[0] as char)
        .collect()
}
