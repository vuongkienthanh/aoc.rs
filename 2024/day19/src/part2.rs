use std::collections::HashMap;

fn parse(input: &str) -> (Vec<&str>, HashMap<&str, usize>, &str) {
    let mut memo = HashMap::<&str, usize>::new();
    let mut initial_patterns = vec![];

    let (pattern_list, towels) = input.split_once("\n\n").unwrap();

    let mut lens = HashMap::<usize, Vec<&str>>::new();
    let mut max_len = 0;
    for pat in pattern_list.split(", ") {
        max_len = max_len.max(pat.len());
        lens.entry(pat.len()).or_default().push(pat);
    }
    for pat in lens.get(&1).unwrap() {
        memo.insert(pat, 1);
        initial_patterns.push(*pat);
    }

    for i in 2..max_len + 1 {
        for pat in lens.get(&i).unwrap() {
            counting(pat, &mut memo, &initial_patterns);
            *memo.get_mut(pat).unwrap() += 1;
            initial_patterns.push(*pat);
        }
    }
    (initial_patterns, memo, towels)
}

pub fn process(_input: &str) -> usize {
    let (initial_patterns, mut memo, towels) = parse(_input);

    towels
        .lines()
        .map(|towel| counting(towel, &mut memo, &initial_patterns))
        .sum()
}
fn counting<'a>(
    towel: &'a str,
    memo: &mut HashMap<&'a str, usize>,
    initial_patterns: &[&str],
) -> usize {
    if let Some(ans) = memo.get(&towel) {
        *ans
    } else {
        let ans = initial_patterns
            .iter()
            .filter_map(|pat| towel.strip_prefix(pat))
            .map(|sub| counting(sub, memo, initial_patterns))
            .sum();
        memo.insert(towel, ans);
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"r, wr, b, g, bwu, rb, gb, br

bggr
brwrr
bwurrg
gbbr
rrbgbr
ubwu
bbrgwb
brgr
"#;
        assert_eq!(process(input), 16);
    }
}
