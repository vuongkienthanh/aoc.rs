use crate::evolve;
use std::collections::{HashMap, HashSet};

pub fn process(_input: &str) -> usize {
    _input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .fold(HashMap::<[isize; 4], isize>::new(), |mut acc, mut x| {
            let mut digit = (x % 10) as isize;
            let mut changes = [0; 4];
            for i in &mut changes {
                let new_x = evolve(x);
                let new_digit = (new_x % 10) as isize;
                *i = new_digit - digit;
                x = new_x;
                digit = new_digit;
            }
            let mut seen = HashSet::from([changes]);
            *acc.entry(changes).or_default() += digit;

            for _ in 0..1996 {
                let new_x = evolve(x);
                let new_digit = (new_x % 10) as isize;
                changes = [changes[1], changes[2], changes[3], new_digit - digit];

                if seen.insert(changes) {
                    *acc.entry(changes).or_default() += new_digit;
                }
                x = new_x;
                digit = new_digit;
            }
            acc
        })
        .into_values()
        .max()
        .unwrap() as usize
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"1
2
3
2024"#;
        assert_eq!(process(input), 23);
    }
}
