use std::collections::HashMap;

pub fn process(_input: &str) -> usize {
    let mut categories = HashMap::<char, Vec<&str>>::new();
    let mut memo = HashMap::<&str, bool>::new();

    let (pattern_cat, puzzle) = _input.split_once("\n\n").unwrap();
    for pat in pattern_cat.split(", ") {
        let c = pat.chars().next().unwrap();
        categories.entry(c).or_default().push(pat);
        memo.insert(pat, true);
    }
    if cfg!(test) {
        println!("memo = {memo:?}");
        println!("categories = {categories:?}");
    }

    puzzle
        .lines()
        .filter(|line| is_possible(line, &mut categories, &mut memo))
        .count()
}

fn is_possible<'a>(
    line: &'a str,
    categories: &mut HashMap<char, Vec<&'a str>>,
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    if line.is_empty() {
        true
    } else if let Some(ans) = memo.get(&line) {
        *ans
    } else {
        let first_char = line.chars().next().unwrap();
        if let Some(patterns) = categories.get(&first_char).cloned() {
            for pat in patterns {
                if let Some(rest) = line.strip_prefix(pat) {
                    if is_possible(rest, categories, memo) {
                        categories.get_mut(&first_char).unwrap().push(line);
                        memo.insert(line, true);
                        return true;
                    }
                }
            }
            memo.insert(line, false);
            false
        } else {
            false
        }
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
