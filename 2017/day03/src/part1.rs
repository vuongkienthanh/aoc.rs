use crate::generate;
pub fn process(_input: &str) -> usize {
    let input: usize = _input.parse().unwrap();
    let mut end = 1usize;
    for layer in 1.. {
        let range = end + 1..=end + 8 * layer;
        if range.contains(&input) {
            return range
                .zip(generate(layer))
                .find_map(|(ele, (p0, p1))| {
                    (ele == input).then(|| p0.unsigned_abs() + p1.unsigned_abs())
                })
                .unwrap();
        }
        end = *range.end();
    }

    panic!("should have an answer")
}
