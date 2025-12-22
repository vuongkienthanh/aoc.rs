pub mod parsing;
pub mod part1;
pub mod part2;

use parsing::Graph;
use std::collections::VecDeque;

fn calculate_happiness(input: &[&str], graph: &Graph) -> isize {
    input
        .windows(2)
        .map(|v| {
            graph.get(v[0]).unwrap().get(v[1]).unwrap()
                + graph.get(v[1]).unwrap().get(v[0]).unwrap()
        })
        .chain({
            let a = input.first().unwrap();
            let b = input.last().unwrap();
            Some(graph.get(a).unwrap().get(b).unwrap() + graph.get(b).unwrap().get(a).unwrap())
        })
        .sum()
}

fn build_all_combinations(mut names: Vec<&str>) -> VecDeque<Vec<&str>> {
    fn build<'a>(vd: &mut VecDeque<Vec<&'a str>>, mut rest: Vec<&'a str>) {
        if let Some(name) = rest.pop() {
            for _ in 0..vd.len() {
                let current = vd.pop_front().unwrap();
                for i in 0..current.len() {
                    let mut next = current[..i].to_vec();
                    next.push(name);
                    next.extend_from_slice(&current[i..]);
                    vd.push_back(next);
                }
            }
            build(vd, rest);
        }
    }
    assert!(names.len() > 3);
    let rest = names.split_off(3);
    let mut vd = VecDeque::from([names]);

    build(&mut vd, rest);
    vd
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::parse_input;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."#
    }
    #[rstest]
    fn test_calculate_happiness(fixture: &str) {
        let graph = parse_input(fixture);
        assert_eq!(
            calculate_happiness(&["Alice", "Bob", "Carol", "David"], &graph),
            330
        );
    }
}
