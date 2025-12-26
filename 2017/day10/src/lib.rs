pub mod part1;
pub mod part2;
pub mod parsing;

pub fn process_input(input: &str) -> Vec<usize> {
    input
        .bytes()
        .chain([17, 31, 73, 47, 23])
        .map(|x| x as usize)
        .collect()
}

pub fn process_output(output: [u8; 256]) -> String {
    output
        .as_slice()
        .chunks(16)
        .map(|x| x.iter().fold(0, |acc, ele| acc ^ ele))
        .map(|x| format!("{x:0>2x}"))
        .collect()
}
pub fn round64(input: Vec<usize>) -> [u8; 256] {
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
