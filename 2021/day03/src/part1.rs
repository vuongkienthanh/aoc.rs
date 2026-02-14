use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let cols = _input.lines().next().unwrap().len();
    let mut input = parse_input(_input);
    let rows = input.len();
    let mut gamma =0;
    let mut epsilon =0;
    for i in 0..cols {
        let mut ones = 0;
        for n in input.iter_mut() {
            if *n & 1 == 1 {
                ones +=1;
            }
            *n >>=1;
        }
        let zeros = rows - ones;
        if ones > zeros {
            gamma |= 1<<i;
        } else {
            epsilon |= 1<<i;
        }
    }
    gamma * epsilon
}