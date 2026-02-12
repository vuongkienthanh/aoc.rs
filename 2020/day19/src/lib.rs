pub mod parsing;
pub mod part1;
pub mod part2;

use fxhash::{FxHashMap, FxHashSet};
use parsing::{Rule, Rules};
pub type Cache = FxHashMap<usize, FxHashSet<String>>;

pub fn find_set(i: usize, rules: &Rules, cache: &mut Cache) -> FxHashSet<String> {
    if let Some(ans) = cache.get(&i).cloned() {
        ans
    } else {
        let mut ans = FxHashSet::default();
        ans.insert(String::new());
        let current = vec![rules.get(&i).cloned().unwrap()];

        for rule in current {
            match rule {
                Rule::Char(c) => {
                    let mut new_ans = FxHashSet::default();
                    for mut s in ans {
                        s.push(c);
                        new_ans.insert(s);
                    }
                    ans = new_ans;
                }
                Rule::Or(v) => {
                    let mut new_ans = FxHashSet::default();
                    for chain in v {
                        let mut ac = ans.clone();
                        for i in chain {
                            let mut mid_ans = FxHashSet::default();
                            for s2 in find_set(i, rules, cache) {
                                for s in &ac {
                                    let mut s = s.clone();
                                    s.push_str(s2.as_str());
                                    mid_ans.insert(s);
                                }
                            }
                            ac = mid_ans;
                        }
                        new_ans.extend(ac);
                    }
                    ans = new_ans;
                }
            }
        }

        cache.insert(i, ans.clone());
        ans
    }
}
