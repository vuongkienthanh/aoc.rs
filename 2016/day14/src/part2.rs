use crate::{find_five, find_three};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

type CharIndices = Vec<(char, usize)>;

pub fn process(_input: &str) -> usize {
    let mut possible_keys: HashMap<char, HashSet<usize>> = HashMap::new();
    let mut count = 0;
    for i in 0.. {
        let (threes, mut fives) = (i * 1000..((i + 1) * 1000))
            .into_par_iter()
            .fold(
                || (CharIndices::new(), CharIndices::new()),
                |(mut threes, mut fives), j| {
                    let input = format!("{_input}{j}");
                    let mut hash = format!("{:x}", md5::compute(input));
                    for _ in 0..2016 {
                        hash = format!("{:x}", md5::compute(hash));
                    }
                    if let Some(c) = find_three(&hash) {
                        threes.push((c, j));
                    }
                    for c in find_five(&hash) {
                        fives.push((c, j));
                    }
                    (threes, fives)
                },
            )
            .reduce(
                || (CharIndices::new(), CharIndices::new()),
                |mut a: (CharIndices, CharIndices), b: (CharIndices, CharIndices)| {
                    a.0.extend(b.0);
                    a.1.extend(b.1);
                    a
                },
            );
        for (k, v) in threes {
            possible_keys.entry(k).or_default().insert(v);
        }

        println!("batch: {i}000 ~ {i}999");
        fives.sort_unstable_by_key(|x| x.1); // sort by index

        let mut found_keys = vec![];
        for (k, v) in fives {
            let possible_keys_for_this_char = possible_keys.get(&k).cloned().unwrap();
            for pk in possible_keys_for_this_char {
                if (v.saturating_sub(1000)..v).contains(&pk) {
                    possible_keys.get_mut(&k).unwrap().remove(&pk);
                    found_keys.push(pk);
                }
            }
        }
        found_keys.sort_unstable();
        for k in found_keys {
            count += 1;
            println!("found key:============{k}===========i={count}");
            if count == 64 {
                return k;
            }
        }
    }
    panic!("should have an answer")
}
