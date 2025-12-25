pub fn process(_input: &str) -> String {
    let input = process_input(_input);
    let output = round64(input);
    process_output(output)
}

fn process_input(input: &str) -> Vec<usize> {
    input
        .bytes()
        .chain([17, 31, 73, 47, 23])
        .map(|x| x as usize)
        .collect()
}

fn process_output(output: [u8; 256]) -> String {
    output
        .as_slice()
        .chunks(16)
        .map(|x| x.iter().fold(0, |acc, ele| acc ^ ele))
        .map(|x| format!("{x:0>2x}"))
        .collect()
}
fn round64(input: Vec<usize>) -> [u8; 256] {
    let mut list = [0u8; 256];
    (0..256).for_each(|i| list[i] = i as u8);
    let mut i = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        for length in &input {
            let half = length / 2;
            for left in 0..half {
                let right = length - 1 - left;
                list.swap((i + left) % 256, (i + right) % 256);
            }
            i = (i + length + skip_size) % 256;
            skip_size += 1;
        }
    }

    list
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("", "a2582a3a0e66e6e86e3812dcb672a272".to_string())]
    #[case("AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd".to_string())]
    #[case("1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d".to_string())]
    #[case("1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e".to_string())]
    fn test_process(#[case] input: &str, #[case] expect: String) {
        assert_eq!(process(input), expect);
    }
}
