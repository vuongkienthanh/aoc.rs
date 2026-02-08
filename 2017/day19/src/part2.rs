use crate::parsing::parse_input;
use crate::step;
use aoc_helper::direction::Direction;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let (mut p, mut dir) = input[0]
        .iter()
        .enumerate()
        .find_map(|(i, c)| (*c == '|').then_some(((0, i), Direction::Down)))
        .unwrap();

    let mut ans = 1;

    while let Some((new_p, new_dir)) = step(p, dir, &input) {
        p = new_p;
        dir = new_dir;
        ans += 1;
    }

    ans
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 38);
    }
}
