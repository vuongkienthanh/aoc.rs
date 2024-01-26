use std::collections::{HashMap, HashSet};

pub fn process(input: &str, steps: usize) -> usize {
    let graph = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| ((i as isize, j as isize), c))
        })
        .filter(|(_, c)| ".S".contains(*c))
        .collect::<HashMap<(isize, isize), char>>();

    let mut done = vec![];
    let mut todo = graph
        .iter()
        .filter(|(_, &v)| v == 'S')
        .map(|(k, _)| *k)
        .collect::<HashSet<(isize, isize)>>();

    for s in 0..(3 * 131) {
        if s == 64 {
            println!("{}", todo.len());
        }
        if s % 131 == 65 {
            done.push(todo.len());
        }
        todo = todo
            .into_iter()
            .flat_map(|coord| {
                [(1isize, 0isize), (-1, 0), (0, 1), (0, -1)]
                    .into_iter()
                    .map(move |offset| (coord.0 + offset.0, coord.1 + offset.1))
            })
            .filter(|coord| {
                let k = cmod(coord.0, coord.1);
                graph.contains_key(&k)
            })
            .collect();
    }

    dbg!(&done);

    let a = done.get(0).unwrap();
    let b = done.get(1).unwrap();
    let c = done.get(2).unwrap();
    let n = steps / 131;
    a + n * (b - a + (n - 1) * (c - b - b + a) / 2)
}

fn cmod(a: isize, b: isize) -> (isize, isize) {
    (((a % 131) + 131) % 131, ((b % 131) + 131) % 131)
}
