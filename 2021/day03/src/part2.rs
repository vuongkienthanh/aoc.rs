use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let cols = _input.lines().next().unwrap().len();
    let input = parse_input(_input);
    let mut o2 = 0;
    let mut co2 = 0;
    let mut o2input = input.clone();
    for i in (0..cols).rev() {
        let mut ones = 0;
        for n in &o2input {
            if *n & (1 << i) == (1 << i) {
                ones += 1;
            }
        }
        let zeros = o2input.len() - ones;
        if ones >= zeros {
            o2input.retain(|x| *x & (1 << i) == (1 << i));
        } else {
            o2input.retain(|x| *x & (1 << i) == 0);
        }
        if o2input.len() == 1 {
            o2 = o2input.into_iter().next().unwrap();
            break;
        }
    }
    let mut co2input = input.clone();
    for i in (0..cols).rev() {
        let mut ones = 0;
        for n in &co2input {
            if *n & (1 << i) == (1 << i) {
                ones += 1;
            }
        }
        let zeros = co2input.len() - ones;
        if zeros <= ones {
            co2input.retain(|x| *x & (1 << i) == 0);
        } else {
            co2input.retain(|x| *x & (1 << i) == (1 << i));
        }
        if co2input.len() == 1 {
            co2 = co2input.into_iter().next().unwrap();
            break;
        }
    }
    o2 as usize * co2 as usize
}
