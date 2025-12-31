use crate::parsing::parse_input;
use crate::{Map, build_map, find_roots};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let map = build_map(input);
    work(5, 60, map)
}

fn work(manpower: usize, minute: usize, map: Map) -> usize {
    let mut queue = find_roots(&map);
    let mut done: Vec<char> = vec![];
    let mut time = 0;
    let mut workers = vec![];
    queue.sort_unstable_by(|a, b| b.cmp(a));
    distribute(&mut queue, &mut workers, manpower, minute);

    loop {
        if workers.is_empty() {
            break time;
        }
        let min_time_required = workers
            .iter()
            .min_by_key(|(_, x)| *x)
            .map(|(_, x)| *x)
            .unwrap();
        time += min_time_required;
        let finish_works = workers
            .extract_if(.., |(_, t)| {
                *t -= min_time_required;
                *t == 0
            })
            .map(|(w, _)| w)
            .collect::<Vec<char>>();

        for work in finish_works {
            queue.extend(map.get(&work).unwrap().children.iter().cloned());
            done.push(work);
        }
        queue = workable(queue, &map, &done);
        distribute(&mut queue, &mut workers, manpower, minute);
    }
}

fn work_to_time_required(w: char, minute: usize) -> usize {
    (w as u8 - b'A') as usize + 1 + minute
}

fn workable(mut queue: Vec<char>, map: &Map, done: &[char]) -> Vec<char> {
    queue.sort_unstable_by(|a, b| b.cmp(a));
    queue.dedup();
    queue
        .into_iter()
        .filter(|x| map.get(x).unwrap().parent.iter().all(|p| done.contains(p)))
        .collect::<Vec<char>>()
}

fn distribute(
    queue: &mut Vec<char>,
    workers: &mut Vec<(char, usize)>,
    manpower: usize,
    minute: usize,
) {
    while workers.len() < manpower {
        if let Some(w) = queue.pop() {
            let time_required = work_to_time_required(w, minute);
            workers.push((w, time_required));
        } else {
            break;
        }
    }
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
        let input = parse_input(fixture);
        let map = build_map(input);
        assert_eq!(work(2, 0, map), 15);
    }
}
