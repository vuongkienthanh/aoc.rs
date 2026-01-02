use crate::parsing::parse_input;
use crate::{diffuseness, display, step};

pub fn process(_input: &str) -> String {
    let (p, v) = parse_input(_input);
    let origin_p = p.clone();

    // look at input
    // first digit is always 10_000 * velocity
    // shrink 10_000
    let mut p = step(&p, &v, 10_000);
    
    // diffuseness is the width * height of bounding box, smaller is better
    let (mut second, mut min_d) = (0, usize::MAX);

    // second digit is always less than or equal to first digit
    // so 1000 is reasonable steps to check
    for i in 0..1000 {
        let d = diffuseness(&p);
        if d < min_d {
            min_d = d;
            second = i;
        }
        p = step(&p, &v, 1);
    }
    display(&step(&origin_p, &v, second + 10_000));
    println!("min diffuseness = {min_d}");
    format!("time in second needed to appear = {}", second + 10_000)
}
