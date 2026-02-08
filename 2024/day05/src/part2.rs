use super::parse;
use std::cmp::Ordering;
pub fn process(_input: &str) -> usize {
    let (_, (rules, updates)) = parse(_input).unwrap();
    updates
        .into_iter()
        .filter(|update| {
            update
                .windows(2)
                .any(|x| rules[&x[0]][&x[1]] == Ordering::Greater)
        })
        .map(|mut update| {
            update.sort_unstable_by(|a, b| rules[a][b]);
            update
        })
        .map(|update| {
            let mid = update.len() / 2;
            update
                .into_iter()
                .nth(mid)
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        assert_eq!(process(input), 123);
    }
}
