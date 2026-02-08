use std::collections::HashMap;
use std::collections::HashSet;
pub fn process(_input: &str) -> usize {
    let mut memo = HashMap::<&str, HashSet<&str>>::new();
    let mut ans = 0;
    for line in _input.lines() {
        let (left, right) = line.split_once("-").unwrap();
        memo.entry(left).or_default().insert(right);
        memo.entry(right).or_default().insert(left);

        for intersection in memo
            .get(&left)
            .unwrap()
            .intersection(memo.get(&right).unwrap())
        {
            if [left, right, *intersection]
                .into_iter()
                .any(|x| x.starts_with("t"))
            {
                ans += 1;
            }
        }
    }
    ans
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;
        assert_eq!(process(input), 7);
    }
}
