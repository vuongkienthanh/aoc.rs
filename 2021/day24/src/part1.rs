use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    for g in input {
        println!("{g:?}");
    }
    todo!()
}

fn valid(v: [isize; 14], input: &[(isize, isize, isize)]) -> bool {
    let mut z = 0;
    for (i, (a, b, c)) in input.iter().enumerate() {
        let w = v[i];
        let mut x = z % 26 + b;
        z /= a;
        x = (x != w).into();
        let mut y = 25 * x + 1;
        z *= y;
        y = (w + c) * x;
        z += y;
    }
    z == 0
}
