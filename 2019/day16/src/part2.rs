use crate::parse_u8;

pub fn process(_input: &str) -> usize {
    let offset = _input[..7].parse::<usize>().unwrap();
    let input = parse_u8(_input);

    let len = input.len() * 10_000 - offset;
    let mut meta: Vec<usize> = (1..=len).collect();
    for _phase in 3..=100 {
        for i in 1..len {
            meta[i] = (meta[i - 1] + meta[i]) % 10;
        }
    }
    let mut ans = 0;
    for i in 0..8 {
        ans = ans * 10
            + (offset + i..input.len() * 10_000)
                .into_iter()
                .map(|x| input[x % input.len()])
                .zip(meta.iter())
                .map(|(a, b)| a as usize * *b)
                .sum::<usize>()
                % 10;
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
