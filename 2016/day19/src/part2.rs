use std::collections::VecDeque;
pub fn process(_input: &str) -> usize {
    // let input = _input.parse::<usize>().unwrap();

    // let mut elfs: Vec<usize> = (1..input + 1).collect();
    //
    // while elfs.len() > 1 {
    //     println!("begin {elfs:?}");
    //     if elf.len().is_multiple_of(2) {
    //         let mut new_elfs = elfs[..elfs.len() + 1].to_vec();
    //         new_elfs.extend(elfs[(elfs.len() + 2)..]);
    //     } else {
    //         let mut new_elfs = elfs[..elfs.len()].to_vec();
    //         new_elfs.extend(elfs[(elfs.len() + 1)..]);
    //     }
    //
    //     elfs = new_elfs;
    //     println!("after {elfs:?}");
    // }

    // elfs[0]

    for i in 1..101 {
        let mut elfs: VecDeque<usize> = (1..i + 1).collect();
        while elfs.len() > 1 {
            let half_len = elfs.len() / 2;
            elfs.remove(half_len);
            unsafe {
                let first = elfs.pop_front().unwrap_unchecked();
                elfs.push_back(first);
            }
        }

        println!("elfs len = {i} ; last elf={x} === {i:b} {x:b} ", x = elfs[0]);
    }

    todo!()
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
