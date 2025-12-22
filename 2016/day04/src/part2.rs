use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    for (name, sector, _) in input {
        let d = decrypt(name, sector);
        if d.contains("northpole") {
            return sector;
        }
    }

    todo!("part2")
}

fn decrypt(name: &str, sector: usize) -> String {
    let times = (sector % 26) as u8;
    name.bytes()
        .map(|c| match c {
            b'-' => ' ',
            b'a'..=b'z' => (((c - b'a' + times) % 26) + b'a') as char,
            _ => panic!("should be 'a'..='z' and '-'"),
        })
        .collect()
}
