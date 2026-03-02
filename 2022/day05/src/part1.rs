use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (mut crates, instructions) = parse_input(_input);
    for (i, from, to) in instructions {
        for _ in 0..i {
            let c = crates[from - 1].pop().unwrap();
            crates[to - 1].push(c);
        }
    }
    crates.into_iter().fold(String::new(), |mut acc, mut col| {
        acc.push(col.pop().unwrap());
        acc
    })
}