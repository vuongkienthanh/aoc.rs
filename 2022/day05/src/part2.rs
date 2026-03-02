use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (mut crates, instructions) = parse_input(_input);
    for (i, from, to) in instructions {
        let len = crates[from-1].len();
        let c = crates[from - 1].split_off(len - i);
        crates[to - 1].extend(c);
    }
    crates.into_iter().fold(String::new(), |mut acc, mut col| {
        acc.push(col.pop().unwrap());
        acc
    })
}
