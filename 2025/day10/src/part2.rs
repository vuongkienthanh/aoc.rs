use crate::parsing::parse_input;
// use rayon::prelude::*;
use fxhash::FxHashMap;
use itertools::Itertools;
type Cache = FxHashMap<usize, Vec<Vec<usize>>>;

fn f(buttons: Vec<usize>, mut target: Vec<usize>, cache: &mut Cache) -> Option<usize> {
    println!("target = {target:?}");
    if target.iter().all(|x| *x == 0) {
        return Some(0);
    }
    let odds: usize = target.iter().enumerate().fold(0, |mut acc, (i, ele)| {
        if !ele.is_multiple_of(2) {
            acc |= 1 << i;
        }
        acc
    });
    println!("odd {odds:b}");
    if odds == 0 {
        for ele in target.iter_mut() {
            *ele /= 2;
        }
        return f(buttons, target, cache).map(|x| x * 2);
    }
    let patterns = if let Some(x) = cache.get(&odds) {
        x.clone()
    } else {
        let mut patterns: Vec<Vec<usize>> = vec![];
        let mut i = 1;
        loop {
            for comb in buttons.iter().combinations(i) {
                let mut bulbs = 0;
                for b in &comb {
                    bulbs ^= *b;
                }

                if bulbs == odds {
                    patterns.push(comb.into_iter().cloned().collect());
                }
            }
            i += 1;
            if i > buttons.len() {
                break;
            }
        }
        cache.insert(odds, patterns.clone());
        patterns
    };
    if patterns.is_empty() {
        println!("no pattern found");
        return None;
    }
    for buttons_used in &patterns {
        println!("pattern found: {buttons_used:?} ");
    }
    let mut min = usize::MAX;
    'p: for buttons_used in patterns {
        let mut target = target.clone();
        let buttons_used_len = buttons_used.len();

        print!("{buttons_used:?}:  ");
        for b in &buttons_used {
            print!(" {b:b}");
        }
        println!();

        for mut b in buttons_used {
            let mut i = 0;
            while b  >0 {
                if (b & 1) == 1 {
                    match target[i].checked_sub(1) {
                        Some(x) => target[i] = x,
                        None => continue 'p,
                    }
                }
                i += 1;
                b >>= 1;
            }
        }
        println!("after buttons_used = {target:?}");

        assert!(target.iter().all(|x| x.is_multiple_of(2)));
        for ele in target.iter_mut() {
            *ele /= 2;
        }
        println!("before min = {min}");
        if let Some(m) = f(buttons.clone(), target, cache) {
            min = min.min(buttons_used_len + 2 * m);
        }
        println!("after min = {min}");
    }
    println!("found min afterall patterns= {min}");

    if min == usize::MAX { None } else { Some(min) }
}

fn solve(buttons: Vec<usize>, target: Vec<usize>) -> usize {
    let mut cache = Cache::default();

    f(buttons, target, &mut cache).expect("should have answer")
}

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    input
        // .into_par_iter()
        .into_iter()
        .enumerate()
        .skip(13)
        .take(1)
        .map(|(input_row, (_, buttons, target))| {
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

            let ans = solve(buttons, target);
            println!("done row{input_row}");
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
