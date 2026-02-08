use crate::{parse, tree};

pub fn process(_input: &str) -> usize {
    let (line_len, input) = parse(_input);
    tree(&input, line_len, 3, 1)
}
