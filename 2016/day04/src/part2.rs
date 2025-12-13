use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (_, input) = parse_input(_input).unwrap();

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
    name.chars()
        .map(|c| match c {
            '-' => ' ',
            'a'..='z' => (((c as u8 - 'a' as u8 + times) % 26) + 'a' as u8) as char,
            _ => panic!("should be 'a'..='z' and '-'"),
        })
        .collect()
}
