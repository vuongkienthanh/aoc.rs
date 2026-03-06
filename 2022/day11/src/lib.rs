pub mod parsing;
pub mod part1;
pub mod part2;

#[derive(Debug)]
pub enum Op {
    Add(usize),
    Mul(usize),
    Square,
}
