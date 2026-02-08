use crate::parsing::parse_input;
use itertools::Itertools;
use rayon::prelude::*;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    input
        .into_par_iter()
        .map(|(target, buttons, _)| {
            let buttons: Vec<usize> = buttons
                .into_iter()
                .map(|x| {
                    let mut ans = 0;
                    for c in x {
                        ans |= 1 << c;
                    }
                    ans
                })
                .collect();
            let mut i = 1;

            'a: loop {
                for comb in buttons.iter().combinations(i) {
                    let mut bulbs = 0;
                    for b in comb {
                        bulbs ^= b;
                    }

                    if bulbs == target {
                        break 'a i;
                    }
                }
                i += 1;
                if i > buttons.len() {
                    panic!("should have an answer");
                }
            }
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 7);
    }
}
