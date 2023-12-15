use crate::parse_input;

pub fn process(_input: &str) -> usize {
    let (instruction, pair_map) = parse_input(_input);
    let mut dst = pair_map
        .keys()
        .filter_map(|k| k.ends_with('A').then_some(*k))
        .collect::<Vec<&str>>();
    instruction
        .cycle()
        .take_while(|i| {
            dst = match i {
                'L' => dst
                    .iter()
                    .map(|loc| pair_map.get(loc).unwrap().left)
                    .collect::<Vec<_>>(),
                'R' => dst
                    .iter()
                    .map(|loc| pair_map.get(loc).unwrap().right)
                    .collect::<Vec<_>>(),
                _ => unreachable!(),
            };
            dbg!(&dst);
            !dst.iter().all(|loc| loc.ends_with('Z'))
        })
        .count()
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
        assert_eq!(process(input), 6);
    }
}
