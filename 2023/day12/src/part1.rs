use crate::recurse;
use std::collections::HashMap;

pub fn process(_input: &str) -> usize {
    let mut ret = 0;
    for line in _input.lines() {
        let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
        let (lava, springs) = line.split_once(' ').unwrap();
        let springs = springs
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        ret += recurse(&lava, &springs, &mut cache);
    }
    ret
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
        assert_eq!(process(input), 21);
    }
}
