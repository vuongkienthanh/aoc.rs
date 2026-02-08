pub mod part1;
pub mod part2;

use std::collections::HashMap ;
struct Pair<'a> {
    left: &'a str,
    right: &'a str,
}

struct Instruction<'a>(&'a str);
impl<'a> Instruction<'a> {
    fn cycle(&self) -> std::iter::Cycle<std::str::Chars<'a>> {
        self.0.chars().cycle()
    }
}

fn parse_input<'a>(_input: &'a str) -> (Instruction<'a>, HashMap<&'a str, Pair<'a>>) {
    let mut split = _input.split("\n\n");
    let instruction = Instruction(split.next().unwrap());
    let pair_map = split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut linesplit = line.split(" = ");
            let name = linesplit.next().unwrap();
            let paren = linesplit.next().unwrap();
            let mut pairsplit = paren.get(1..(paren.len() - 1)).unwrap().split(", ");
            let pair = Pair {
                left: pairsplit.next().unwrap(),
                right: pairsplit.next().unwrap(),
            };
            (name, pair)
        })
        .collect::<HashMap<&str, Pair>>();

    (instruction, pair_map)
}
