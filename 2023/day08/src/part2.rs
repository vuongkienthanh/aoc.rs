use crate::parse_input;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}
pub fn process(_input: &str) -> usize {
    let (instruction, pair_map) = parse_input(_input);
    lcm(&pair_map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| {
            let mut dst = *k;
            instruction
                .cycle()
                .take_while(|i| {
                    dst = match i {
                        &'L' => pair_map.get(&dst).unwrap().left,
                        &'R' => pair_map.get(&dst).unwrap().right,
                        _ => unreachable!(),
                    };
                    !dst.ends_with('Z')
                })
                .count()
                + 1
        })
        .collect::<Vec<usize>>())
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
