pub mod part1;
pub mod part2;
fn compact(input: Vec<char>) -> Vec<usize> {
    let mut continuous = 0;
    let mut ret = vec![];
    for c in input.iter() {
        if *c == '#' {
            continuous += 1;
        } else if continuous != 0 {
            ret.push(continuous);
            continuous = 0;
        }
    }
    if continuous != 0 {
        ret.push(continuous);
    }
    ret
}
