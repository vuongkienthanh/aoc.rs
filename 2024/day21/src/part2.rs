use crate::min_keypresses;
pub fn process(_input: &str) -> usize {
    _input
        .lines()
        .map(|line| {
            let keypresses = min_keypresses(line, 25);
            let code = line[0..3].parse::<usize>().unwrap();
            code * keypresses
        })
        .sum()
}
