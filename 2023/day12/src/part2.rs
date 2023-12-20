use std::ops::Mul;

use crate::compact;
use itertools::Itertools;

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

        let damaged_spring_needed =
            right.iter().sum::<usize>() - left.iter().filter(|x| **x == '#').count();

        // original
        let mut origin_line_ret = 0usize;
        let unknowns = left
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if *c == '?' { Some(i) } else { None })
            .collect::<Vec<usize>>();
        for each_comb in unknowns.iter().combinations(damaged_spring_needed) {
            let mut left_clone = left.clone();
            for idx in each_comb {
                *left_clone.get_mut(*idx).unwrap() = '#';
            }
            if compact(left_clone) == right {
                origin_line_ret += 1;
            }
        }

        let mut first_case = left.clone();
        first_case.push('?');
        let mut first_case_line_ret = 0usize;
        let first_case_unknowns = first_case
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if *c == '?' { Some(i) } else { None })
            .collect::<Vec<usize>>();
        for each_comb in first_case_unknowns.iter().combinations(damaged_spring_needed) {
            let mut clone = first_case.clone();
            for idx in each_comb {
                *clone.get_mut(*idx).unwrap() = '#';
            }
            if compact(clone) == right {
                first_case_line_ret += 1;
            }
        }

        let mut second_case = left.clone();
        second_case.insert(0, '?');
        let mut second_case_line_ret = 0usize;
        let second_case_unknowns = second_case
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if *c == '?' { Some(i) } else { None })
            .collect::<Vec<usize>>();
        for each_comb in second_case_unknowns.iter().combinations(damaged_spring_needed) {
            if second_case.last().unwrap() == &'#' && each_comb.contains(&&0) {
                continue;
            }
            let mut clone = second_case.clone();
            for idx in each_comb {
                *clone.get_mut(*idx).unwrap() = '#';
            }
            if compact(clone) == right {
                second_case_line_ret += 1;
            }
        }
        let line_ret = first_case_line_ret.max(second_case_line_ret);
        ret += line_ret.pow(4).mul(origin_line_ret);
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
        assert_eq!(process(input), 525152);
    }
}
