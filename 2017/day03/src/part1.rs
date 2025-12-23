use crate::Spiral;
pub fn process(_input: &str) -> usize {
    let input: usize = _input.parse().unwrap();
    let mut range = 1..=1usize;
    for layer in 1.. {
        range = range.end() + 1..=range.end() + 8 * layer;
        if range.contains(&input) {
            return range
                .zip(Spiral::generate(layer as isize))
                .find_map(|(ele, (p0, p1))| {
                    (ele == input).then_some(p0.unsigned_abs() + p1.unsigned_abs())
                })
                .unwrap();
        }
    }

    panic!("should have an answer")
}
