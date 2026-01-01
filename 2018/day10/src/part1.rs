use crate::parsing::parse_input;
use crate::{compactness, display, shrink_10000, step};

pub fn process(_input: &str) -> String {
    let (mut p, v) = parse_input(_input);

    // look at input
    // first digit is always 10_000 * velocity
    shrink_10000(&mut p, &v);

    // pick threshold for compactness to reduce cloning
    // compactness is number of manhatan distance that == 1
    // number of points is good
    let (mut target, mut second, mut max_c) = (vec![], 0, p.len());

    // second digit is always less than or equal to first digit
    // so 1000 is reasonable steps to check
    for i in 0..1000 {
        let c = compactness(&p);
        if c > max_c {
            max_c = c;
            target = p.clone();
            second = i;
        }
        p = step(&p, &v);
    }
    display(&target);
    println!("input len = {}", p.len());
    println!("max compactness = {max_c}");
    format!("time in second needed to appear = {}", second + 10_000)
}
