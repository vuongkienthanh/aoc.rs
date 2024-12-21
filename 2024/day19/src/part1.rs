use std::collections::HashMap;

pub fn process(_input: &str) -> usize {
    let mut memo = HashMap::<&str, bool>::new();

    let (pattern_list, towels) = _input.split_once("\n\n").unwrap();
    for pat in pattern_list.split(", ") {
        memo.insert(pat, true);
    }
    let initial_patterns = memo.keys().cloned().collect::<Vec<_>>();

    towels
        .lines()
        .filter(|line| is_possible(line, &mut memo, &initial_patterns))
        .count()
}

fn is_possible<'a>(
    towel: &'a str,
    memo: &mut HashMap<&'a str, bool>,
    initial_patterns: &[&str],
) -> bool {
    if let Some(ans) = memo.get(&towel) {
        *ans
    } else {
        for sub in initial_patterns
            .iter()
            .filter_map(|pat| towel.strip_prefix(pat))
        {
            if is_possible(sub, memo, initial_patterns) {
                memo.insert(towel, true);
                return true;
            }
        }
        memo.insert(towel, false);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;
        assert_eq!(process(input), 6);
    }
}
