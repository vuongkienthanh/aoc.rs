use crate::part1::mask;

pub fn process(_input: &str) -> u32 {
    let input: Vec<_> = _input.lines().collect();
    let mut ans = 0;

    for group in input.chunks(3) {
        let elf0 = group[0].bytes().fold(0u64, |acc, ele| acc | mask(ele));
        let elf1 = group[1].bytes().fold(0u64, |acc, ele| acc | mask(ele));
        let elf2 = group[2].bytes().fold(0u64, |acc, ele| acc | mask(ele));
        let badge = elf0 & elf1 & elf2;
        ans += badge.ilog(2);
    }
    ans
}
