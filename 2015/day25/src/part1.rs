const TARGET: (usize, usize) = (2978, 3083);
const MUL: usize = 252533;
const MOD: usize = 33554393;

pub fn process(_input: &str) -> usize {
    let mut i = 1;
    let mut j = 1;
    let mut current_number = 20151125;
    let mut max_i = 1;

    while (i, j) != TARGET {
        if i == 1 {
            i = max_i + 1;
            max_i = i;
            j = 1;
        } else {
            i -= 1;
            j += 1;
        }

        current_number = next(current_number);
    }
    current_number
}

fn next(n: usize) -> usize {
    (n * MUL) % MOD
}
