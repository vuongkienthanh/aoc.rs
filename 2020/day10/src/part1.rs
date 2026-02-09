use crate::parse;

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    let mut ones = 0;
    let mut threes = 0;
    for v in input.windows(2) {
        if v[1] - v[0] == 1 {
            ones += 1;
        } else if v[1] - v[0] == 3 {
            threes += 1;
        }
    }
    ones * threes
}
