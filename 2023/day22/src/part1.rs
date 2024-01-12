use crate::{parse_input, Tower};

impl Tower {
    fn is_safely_remove_brick(&self, brick_name: usize) -> bool {
        let mut support = self.support.clone();
        if let Some(supported_bricks) = support.remove(&brick_name) {
            for brick in supported_bricks {
                // not supported by some other bricks
                if !support.values().any(|v| v.contains(&brick)) {
                    return false; // not safe
                }
            }
        }
        return true; // safe
    }
}

pub fn process(_input: &str) -> usize {
    let parsed = parse_input(_input);
    let tower = Tower::new(parsed);
    let bricks: Vec<usize> = tower.support.keys().map(|c| *c).collect();
    bricks
        .into_iter()
        .filter(|n| tower.is_safely_remove_brick(*n))
        .count()
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
        assert_eq!(process(input), 5);
    }
}
