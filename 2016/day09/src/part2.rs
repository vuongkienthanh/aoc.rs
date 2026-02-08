pub fn process(_input: &str) -> usize {
    let mut ans = 0;
    let mut i = _input.chars();
    while let Some(c) = i.next() {
        match c {
            '(' => {
                let mut left = String::new();
                for c2 in i.by_ref() {
                    if c2 == 'x' {
                        break;
                    } else {
                        left.push(c2);
                    }
                }
                let mut right = String::new();
                for c2 in i.by_ref() {
                    if c2 == ')' {
                        break;
                    } else {
                        right.push(c2);
                    }
                }
                let chars_count = left.parse::<usize>().unwrap();
                let repeat = right.parse::<usize>().unwrap();
                let mut within = String::new();
                for _ in 0..chars_count {
                    within.push(i.next().unwrap());
                }
                ans += process(&within) * repeat;
            }
            _ => ans += 1,
        }
    }
    ans
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("X(8x2)(3x3)ABCY", "XABCABCABCABCABCABCY".len())]
    #[case("(27x12)(20x12)(13x14)(7x10)(1x12)A", 241920)]
    #[case("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN", 445)]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
