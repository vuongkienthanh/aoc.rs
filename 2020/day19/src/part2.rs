use crate::parsing::{Rule, parse_input};
use fxhash::{FxHashMap, FxHashSet};

type Cache = FxHashMap<usize, FxHashSet<String>>;
type Cache2<'a> = FxHashMap<&'a str, Vec<(usize, usize)>>;

pub fn process(_input: &str) -> usize {
    let (map, msg) = parse_input(_input);
    let mut cache = Cache::default();

    let g31 = find_set(31, &map, &mut cache);
    let g42 = find_set(42, &map, &mut cache);
    println!("{} {g31:?}", g31.len());
    println!("{} {g42:?}", g42.len());

    // let mut cache2 = Cache2::default();
    // msg.into_iter()
    //     .filter(|line| is_match(line, 0, 0, &g31, &g42, &mut cache2))
    //     .count()
    todo!()
}

fn status<'a>(
    line: &'a str,
    g31: &'a FxHashSet<String>,
    g42: &'a FxHashSet<String>,
    cache: &mut Cache2<'a>,
) -> Vec<(usize, usize)> {
    if let Some(ans) = cache.get(&line).cloned() {
        return ans;
    }
    for end in g31




    if end_count == 0 {
        for end in g31 {
            if line.ends_with(end) {
                let line = &line[..line.len().saturating_sub(end.len())];
                if is_match(line, 0, 1, g31, g42, cache) {
                    return true;
                }
            }
        }
        false
    } else {
        if start_count < end_count {
            if start_count == 0 {
                for end in g31 {
                    if line.ends_with(end) {
                        let line = &line[..line.len().saturating_sub(end.len())];
                        if is_match(line, 0, end_count + 1, g31, g42, cache) {
                            return true;
                        }
                    }
                }
                for start in g42 {
                    if line.ends_with(start) {
                        let line = &line[..line.len().saturating_sub(start.len())];
                        if is_match(line, 1, end_count, g31, g42, cache) {
                            return true;
                        }
                    }
                }
                false
            } else {
                for start in g42 {
                    if line.ends_with(start) {
                        let line = &line[..line.len().saturating_sub(start.len())];
                        if is_match(line, start_count + 1, end_count, g31, g42, cache) {
                            return true;
                        }
                    }
                }
                false
            }
        } else {
            for start in g42 {
                if line.ends_with(start) {
                    let line = &line[..line.len().saturating_sub(start.len())];
                    if is_match(line, start_count, end_count, g31, g42, cache) {
                        return true;
                    }
                }
            }
            false
        }
    }
}

fn find_set(i: usize, rules: &FxHashMap<usize, Rule>, cache: &mut Cache) -> FxHashSet<String> {
    if let Some(ans) = cache.get(&i).cloned() {
        ans
    } else {
        let mut ans = FxHashSet::default();
        ans.insert(String::new());
        let mut current = vec![rules.get(&i).cloned().unwrap()];

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
