use crate::Cell;

pub fn parse_input(input: &str) -> Vec<Vec<Cell>> {
    let mut ans = vec![];
    for line in input.lines() {
        let mut v = vec![];
        for cell in line.chars() {
            v.push(match cell {
                '#' => Cell::Wall,
                '.' => Cell::Blizzard(vec![]),
                '>' => Cell::Blizzard(vec![(Blizzard::Right, false)]),
                '<' => Cell::Blizzard(vec![(Blizzard::Left, false)]),
                'v' => Cell::Blizzard(vec![(Blizzard::Down, false)]),
                '^' => Cell::Blizzard(vec![(Blizzard::Up, false)]),
                _ => panic!(),
            });
        }
        ans.push(v);
    }
    ans
}