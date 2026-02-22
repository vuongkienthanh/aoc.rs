use crate::normalize;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let (_, scanner_loc) = normalize(input);
    let mut max = 0;
    for i in 0..scanner_loc.len() - 1 {
        for j in i + 1..scanner_loc.len() {
            let (a, b, c) = scanner_loc.get(i).unwrap();
            let (x, y, z) = scanner_loc.get(j).unwrap();
            max = max.max(a.abs_diff(*x) + b.abs_diff(*y) + c.abs_diff(*z));
        }
    }
    max
}
