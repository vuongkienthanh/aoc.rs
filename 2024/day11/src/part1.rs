pub fn process(_input: &str, times: usize) -> usize {
    _input
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .map(|x| blink(x, times))
        .sum()
}

fn blink(x: usize, times: usize) -> usize {
    let x_len = x.checked_ilog10().unwrap_or_default() + 1;
    let is_even = x_len % 2 == 0;
    if times == 1 {
        if x == 0 {
            1
        } else if is_even {
            2
        } else {
            1
        }
    } else if x == 0 {
        blink(1, times - 1)
    } else if is_even {
        let mut left = x;
        let mut right = 0;
        for i in 0..x_len / 2 {
            let digit = left % 10;
            right += digit * 10usize.pow(i);
            left /= 10;
        }
        blink(left, times - 1) + blink(right, times - 1)
    } else {
        blink(x * 2024, times - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"125 17"#;
        assert_eq!(process(input, 25), 55312);
    }
    #[test]
    fn test_process3() {
        let input = r#"125 17"#;
        assert_eq!(process(input, 6), 22);
    }
    #[test]
    fn test_process2() {
        let input = r#"0 1 10 99 999"#;
        assert_eq!(process(input, 1), 7);
    }
}
