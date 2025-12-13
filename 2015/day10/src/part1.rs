use crate::next;
pub fn process(_input: &str) -> usize {
    let mut input = String::from(_input);

    for _ in 0..40 {
        input = next(input);
    }

    input.len()
}
