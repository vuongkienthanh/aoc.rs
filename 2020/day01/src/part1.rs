use std::cmp::Ordering;
pub fn process(_input: &str) -> usize {
    let mut input: Vec<_> = _input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
    input.sort_unstable();
    let mut ans = 0;
    'a: for a in &input {
        for b in input.iter().rev() {
            match (a + b).cmp(&2020) {
                Ordering::Less => break,
                Ordering::Greater => (),
                Ordering::Equal => {
                    ans = a * b;
                    break 'a;
                }
            }
        }
    }
    ans
}
