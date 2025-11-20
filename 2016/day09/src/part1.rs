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
                for _ in 0..chars_count {
                    i.next().unwrap();
                }
                ans += chars_count * repeat;
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
    #[case("ADVENT", 6)]
    #[case("A(1x5)BC", 7)]
    #[case("(3x3)XYZ", 9)]
    #[case("A(2x2)BCD(2x2)EFG", 11)]
    #[case("(6x1)(1x3)A", 6)]
    #[case("X(8x2)(3x3)ABCY", 18)]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
