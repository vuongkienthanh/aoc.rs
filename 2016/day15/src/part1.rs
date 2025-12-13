use crate::modular_inverse;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (_rest, input) = parse_input(_input).unwrap();
    println!("Chinese Remainder Theorem:");
    for i in &input {
        println!("x = {} (mod {})", i.rem, i.modulus);
    }
    assert!(_rest.is_empty());

    let modulus_product = input.iter().map(|x| x.modulus).product::<usize>();
    let identities = input
        .iter()
        .map(|x| modulus_product / x.modulus)
        .collect::<Vec<usize>>();
    let ys = input
        .iter()
        .zip(identities.iter())
        .map(|(x, y)| modular_inverse(*y, x.modulus).unwrap())
        .collect::<Vec<usize>>();

    println!("\nM = {modulus_product}\n");
    for (i, id) in identities.iter().enumerate() {
        println!("M{i} = {id}");
    }
    println!();
    for (i, v) in ys.iter().enumerate() {
        println!("y{i} = {v}");
    }
    println!();

    input
        .into_iter()
        .zip(identities)
        .zip(ys)
        .map(|((a, b), c)| a.rem * b * c)
        .sum::<usize>()
        % modulus_product
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1."#
    }
    #[rstest]
    fn test_process(fixture: &str) {
        assert_eq!(process(fixture), 5);
    }
    #[rstest]
    #[case(35, 3, 2)]
    #[case(21, 5, 1)]
    #[case(15, 7, 1)]
    fn test_modular_inverse(#[case] a: usize, #[case] m: usize, #[case] expected: usize) {
        assert_eq!(modular_inverse(a, m), Some(expected));
    }
}
