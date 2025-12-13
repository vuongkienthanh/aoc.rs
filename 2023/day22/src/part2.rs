use std::collections::VecDeque;
use crate::{parse_input, Tower};

impl Tower {
    fn disintegrate_and_fall(&mut self, brick_name: usize) -> usize {
        let mut support = self.support.clone();
        let mut stack = VecDeque::from([brick_name]);
        let mut ret = 0;
        while let Some(next_brick) = stack.pop_front() {
            if let Some(supported_bricks) = support.remove(&next_brick) {
                for brick in supported_bricks {
                    // not supported by some other bricks
                    if !support.values().any(|v| v.contains(&brick)) {
                        // add to fallen list
                        ret += 1;
                        stack.push_back(brick);
                    }
                }
            }
        }
        ret
    }
}

pub fn process(_input: &str) -> usize {
    let parsed = parse_input(_input);
    let mut tower = Tower::new(parsed);
    let bricks: Vec<usize> = tower.support.keys().map(|c| *c).collect();
    bricks
        .into_iter()
        .map(|n| tower.disintegrate_and_fall(n))
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;
        assert_eq!(process(input), 7);
    }
}
