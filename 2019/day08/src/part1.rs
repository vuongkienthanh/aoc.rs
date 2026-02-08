use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    let mut ans = (usize::MAX, 0);
    for chunk in input.chunks(150) {
        let (mut zeros, mut ones, mut twos) = (0, 0, 0);
        for i in chunk {
            match i {
                0 => zeros += 1,
                1 => ones += 1,
                2 => twos += 1,
                _ => panic!("should be 0 1 2"),
            }
        }
        ans = ans.min((zeros, ones * twos));
    }
    ans.1
}
