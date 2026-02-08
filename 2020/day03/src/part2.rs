use crate::{parse, tree};

pub fn process(_input: &str) -> usize {
    let (line_len, input) = parse(_input);
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(right, down)| tree(&input, line_len, right, down))
        .product()
}
