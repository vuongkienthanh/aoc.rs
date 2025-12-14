use crate::parsing::parse_input;
use fxhash::FxHashMap;
use itertools::Itertools;
use rayon::prelude::*;

type Cache = FxHashMap<Vec<usize>, usize>;

fn f(target: Vec<usize>, patterns: &Vec<(Vec<usize>, usize)>, cache: &mut Cache) -> usize {
    if target.iter().all(|x| *x == 0) {
        return 0;
    }
    if let Some(x) = cache.get(&target) {
        return *x;
    }

    let mut min = 1_000_000;
    for (will_increase, button_count) in patterns {
        if target
            .iter()
            .zip(will_increase)
            .all(|(a, b)| a >= b && a % 2 == b % 2)
        {
            let new_target: Vec<usize> = target
                .iter()
                .zip(will_increase)
                .map(|(a, b)| (a - b) / 2)
                .collect();
            min = min.min(button_count + 2 * f(new_target, patterns, cache))
        }
    }
    cache.insert(target, min);
    min
}

fn solve(buttons: Vec<Vec<usize>>, target: Vec<usize>) -> usize {
    let mut all_patterns: Vec<(Vec<usize>, usize)> = vec![];
    for button_count in 0..=buttons.len() {
        for comb in buttons.iter().combinations(button_count) {
            let mut will_increase = vec![0; target.len()];
            for button in comb {
                for col in button {
                    will_increase[*col] += 1;
                }
            }
            all_patterns.push((will_increase, button_count));
        }
    }
    let mut cache = Cache::default();

    f(target, &all_patterns, &mut cache)
}

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    input
        .into_par_iter()
        .enumerate()
        .map(|(input_row, (_, buttons, target))| {
            let ans = solve(buttons, target);
            println!("done row{input_row} {ans}");
            ans
        })
        .sum::<usize>()
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
        assert_eq!(process(fixture), 33);
    }
}
