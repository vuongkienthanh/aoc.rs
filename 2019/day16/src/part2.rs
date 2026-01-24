use crate::parse_u8;

pub fn process(_input: &str) -> usize {
    let offset = _input[..7].parse::<usize>().unwrap();
    let input = parse_u8(_input);

    let mut real_input = vec![];
    for i in offset..input.len() * 10000 {
        real_input.push(input[i % input.len()]);
    }

    let mut meta: Vec<usize> = (1..=real_input.len()).collect();
    for _phase in 3..=100 {
        for i in 1..meta.len() {
            meta[i] = (meta[i - 1] + meta[i]) % 10;
        }
    }
    let mut ans = 0;
    for i in 0..8 {
        let used_slice = &real_input[i..];
        ans = ans * 10
            + (used_slice
                .iter()
                .zip(meta.iter())
                .map(|(a, b)| *a as usize * *b)
                .sum::<usize>()
                % 10);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("03036732577212944063491565474664", 84462026)]
    #[case("02935109699940807407585447034323", 78725270)]
    #[case("03081770884921959731165446850517", 53553731)]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
