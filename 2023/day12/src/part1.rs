use itertools::Itertools;
use crate::compact;

pub fn process(_input: &str) -> usize {
    let mut ret = 0;
    for line in _input.lines() {
        let mut linesplit = line.split_ascii_whitespace();
        let left = linesplit.next().unwrap().chars().collect::<Vec<char>>();
        let right = linesplit
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let unknowns = left
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if *c == '?' { Some(i) } else { None })
            .collect::<Vec<usize>>();

        let damaged_spring_needed =
            right.iter().sum::<usize>() - left.iter().filter(|x| **x == '#').count();

        for each_comb in unknowns.iter()
            .combinations(damaged_spring_needed)
        {
            let mut left_clone = left.clone();
            for idx in each_comb {
                *left_clone.get_mut(*idx).unwrap() = '#';
            }
            if compact(left_clone) == right {
                ret += 1;
            }
        }
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
