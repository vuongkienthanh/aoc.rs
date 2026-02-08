pub fn process(_input: &str) -> usize {
    // as with Josephus problem
    // I tried to find pattern
    // I found that last_elf increases with elf_len, but when more than half it steps by 2

    // use std::collections::VecDeque;
    // for i in 1..101 {
    //     let mut elfs: VecDeque<usize> = (1..i + 1).collect();
    //     while elfs.len() > 1 {
    //         let half_len = elfs.len() / 2;
    //         elfs.remove(half_len);
    //         unsafe {
    //             let first = elfs.pop_front().unwrap_unchecked();
    //             elfs.push_back(first);
    //         }
    //     }
    //
    //     println!(
    //         "elfs len = {i} ; last elf={x} === {i:b} {x:b} ",
    //         x = elfs[0]
    //     );
    // }

    let input = _input.parse::<usize>().unwrap();
    let mut last_elf = 0;
    for len in 1..(input + 1) {
        if last_elf + 1 > len / 2 {
            last_elf += 2;
        } else {
            last_elf += 1;
        }
        if last_elf > len {
            last_elf = 1;
        }
    }
    last_elf
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("5", 2)]
    #[case("7", 5)]
    #[case("9", 9)]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
