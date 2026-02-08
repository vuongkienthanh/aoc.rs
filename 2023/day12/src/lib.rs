pub mod part1;
pub mod part2;
use std::collections::HashMap;
fn recurse(
    lava: &str,
    springs: &[usize],
    cache: &mut HashMap<(String, Vec<usize>), usize>,
) -> usize {
    // get from cache
    if let Some(ret) = cache.get(&(lava.to_string(), springs.to_vec())) {
        return *ret;
    }

    // base case
    if springs.is_empty() {
        let ret = !lava.contains('#') as usize;
        cache.insert((lava.to_string(), springs.to_vec()), ret);
        return ret;
    }

    // recur case
    let mut ret = 0;
    let current = *springs.get(0).unwrap();
    let new_springs = springs.get(1..).unwrap();

    for i in 0..(lava.len() - new_springs.iter().sum::<usize>() - new_springs.len() - current + 1) {
        if lava[..i].contains('#') {
            break;
        }
        let nxt = i + current;
        if nxt <= lava.len() {
            if !lava[i..nxt].contains('.') {
                if lava.chars().nth(nxt) != Some('#') {
                    if let Some(nxt_lava) = lava.get((nxt + 1)..) {
                        ret += recurse(&nxt_lava, new_springs, cache);
                    } else {
                        ret += recurse("", new_springs, cache);
                    }
                }
            }
        }
    }

    cache.insert((lava.to_string(), springs.to_vec()), ret);
    ret
}
