use crate::{find_five, find_three};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

type CharIndices = HashMap<char, HashSet<usize>>;

pub fn process(_input: &str) -> usize {
    let mut possible_keys: CharIndices = HashMap::new();
    let mut count = 0;
    for i in 0.. {
        let (threes, fives) = (i * 1000..((i + 1) * 1000))
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
                        threes.entry(c).or_default().insert(j);
                    }
                    for c in find_five(&hash) {
                        fives.entry(c).or_default().insert(j);
                    }
                    (threes, fives)
                },
            )
            .reduce(
                || (CharIndices::new(), CharIndices::new()),
                |mut a: (CharIndices, CharIndices), b: (CharIndices, CharIndices)| {
                    for (k, v) in b.0 {
                        a.0.entry(k).or_default().extend(v);
                    }
                    for (k, v) in b.1 {
                        a.1.entry(k).or_default().extend(v);
                    }
                    a
                },
            );
        for (k, v) in threes {
            possible_keys.entry(k).or_default().extend(v)
        }

        println!("batch: {i}000 ~ {i}999");

        let mut found_keys = vec![];
        for (k, v) in fives {
            let mut five_indices = v.into_iter().collect::<Vec<usize>>();
            five_indices.sort_unstable();

            let possible_keys_for_this_char = possible_keys.get(&k).cloned().unwrap();
            for pk in possible_keys_for_this_char {
                for j in &five_indices {
                    if (j.saturating_sub(1000)..*j).contains(&pk) {
                        possible_keys.get_mut(&k).unwrap().remove(&pk);
                        found_keys.push(pk);
                    }
                }
            }
        }
        found_keys.sort_unstable();
        for k in found_keys {
            count += 1;
            println!("==found key:=============={k}=================i={count}");
            if count == 64 {
                return k;
            }
        }
    }
    panic!("no answer")
}
