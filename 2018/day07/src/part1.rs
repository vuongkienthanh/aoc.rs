use crate::parsing::parse_input;
use crate::{build_map, find_roots};

pub fn process(_input: &str) -> String {
    let input = parse_input(_input);
    let map = build_map(input);

    let mut ans: Vec<char> = vec![];

    let mut queue = find_roots(&map);
    queue.sort_unstable_by(|a, b| b.cmp(a));

    while let Some(c) = queue.pop() {
        let node = map.get(&c).unwrap();
        if node.parent.iter().all(|x| ans.contains(x)) {
            ans.push(c);
            queue.extend(node.children.iter().cloned());
            queue.dedup();
            queue.sort_unstable_by(|a, b| b.cmp(a));
        }
    }

    ans.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), "CABDFE");
    }
}
