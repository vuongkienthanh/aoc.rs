pub fn process(_input: &str) -> isize {
    let input: Vec<&str> = _input.lines().collect();
    let mut b = get_last_number(input.iter().nth(0).unwrap());
    let mulb = get_last_number(input.iter().nth(4).unwrap());
    let subb = get_last_number(input.iter().nth(5).unwrap());
    b = b * mulb - subb;
    let subc = get_last_number(input.iter().nth(7).unwrap());
    let c = b - subc;

    let diff = get_last_number(input.iter().rev().nth(1).unwrap()).unsigned_abs();

    let mut h = 0;

    for x in (b..=c).into_iter().step_by(diff) {
        for i in 2..x {
            if x % i == 0 {
                h += 1;
                break;
            }
        }
    }

    h
}

fn get_last_number(input: &str) -> isize {
    input
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap()
}
