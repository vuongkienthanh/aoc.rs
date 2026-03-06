pub mod parsing;
pub mod part1;
pub mod part2;

#[derive(Debug)]
pub enum Op {
    Add(usize),
    Mul(usize),
    Square,
}

pub fn monkey_business(input: Vec<usize>) -> usize {
    let (mut a, mut b) = (0, 0);
    for i in input {
        if i > a {
            b = a;
            a = i;
        } else if i > b {
            b = i;
        }
    }
    a * b
}