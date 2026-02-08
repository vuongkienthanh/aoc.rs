use crate::parse;

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    FFT(input, 100)
}

#[allow(non_snake_case)]
fn FFT(mut input: Vec<isize>, phase: usize) -> usize {
    for _ in 0..phase {
        let mut new = vec![];
        for i in 0..input.len() {
            let used_slice = &input[i..];
            let mut j = 0;
            let mut num = 0;
            'a: loop {
                for _ in 0..i + 1 {
                    if let Some(n) = used_slice.get(j) {
                        num += n;
                        j += 1;
                    } else {
                        break 'a;
                    }
                }
                j += i + 1;
                for _ in 0..i + 1 {
                    if let Some(n) = used_slice.get(j) {
                        num -= n;
                        j += 1;
                    } else {
                        break 'a;
                    }
                }
                j += i + 1;
            }
            new.push((num % 10).abs());
        }

        input = new;
    }
    let ans: Vec<_> = input.into_iter().take(8).collect();
    ans.into_iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (x as usize) * 10usize.pow(i as u32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_fft() {
        let input = parse("12345678");
        assert_eq!(FFT(input, 4), 1029498);
    }

    #[rstest]
    #[case("80871224585914546619083218645595", 24176176)]
    #[case("19617804207202209144916044189917", 73745418)]
    #[case("69317163492948606335995924319873", 52432133)]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
