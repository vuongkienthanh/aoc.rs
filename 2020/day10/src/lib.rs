pub mod part1;
pub mod part2;

pub fn parse(input: &str) -> Vec<usize> {
    let mut v: Vec<_> = input.lines().map(|x| x.parse::<usize>().unwrap()).collect();
    v.push(0);
    v.sort_unstable();
    v.push(v.last().cloned().unwrap() + 3);
    v
}
