use crate::parse_input;

const START: &str = "AAA";
const END: &str = "ZZZ";

pub fn process(_input: &str) -> usize {
    let (instruction, pair_map) = parse_input(_input);
    let mut dst = START;
    instruction
        .cycle()
        .take_while(|i| {
            dst = match i {
                'L' => pair_map.get(&dst).unwrap().left,
                'R' => pair_map.get(&dst).unwrap().right,
                _ => unreachable!(),
            };
            dst != END
        })
        .count()
        + 1
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
        assert_eq!(process(input), 6);
    }
}
