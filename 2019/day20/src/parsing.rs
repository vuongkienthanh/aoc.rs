use std::collections::BTreeMap;

type Point = (usize, usize);

#[derive(Debug, Clone)]
pub enum Cell {
    Space,
    Wall,
    Portal,
}

pub fn parse(input: &str) -> (Vec<Vec<Cell>>, BTreeMap<(char, char), Vec<Point>>) {
    let rows = input.lines().count() - 4;
    let cols = input.lines().next().unwrap().len() - 4;
    let mut ans = vec![vec![Cell::Space; cols]; rows];
    let mut portals: BTreeMap<(char, char), Vec<Point>> = BTreeMap::new();

    let input: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    for row in 1..rows + 3 {
        for col in 1..cols + 3 {
            let (m, u, d, l, r) = (
                input[row][col],
                input[row - 1][col],
                input[row + 1][col],
                input[row][col - 1],
                input[row][col + 1],
            );
            if m.is_alphabetic() {
                if u.is_alphabetic() && d == '.' {
                    ans[row - 1][col - 2] = Cell::Portal;
                    portals.entry((u, m)).or_default().push((row - 1, col - 2));
                    continue;
                }
                if u == '.' && d.is_alphabetic() {
                    ans[row - 3][col - 2] = Cell::Portal;
                    portals.entry((m, d)).or_default().push((row - 3, col - 2));
                    continue;
                }
                if l.is_alphabetic() && r == '.' {
                    ans[row - 2][col - 1] = Cell::Portal;
                    portals.entry((l, m)).or_default().push((row - 2, col - 1));
                    continue;
                }
                if l == '.' && r.is_alphabetic() {
                    ans[row - 2][col - 3] = Cell::Portal;
                    portals.entry((m, r)).or_default().push((row - 2, col - 3));
                    continue;
                }
            } else if m == '#' {
                ans[row - 2][col - 2] = Cell::Wall;
            }
        }
    }

    // for row in &ans {
    //     for cell in row {
    //         match cell {
    //             Cell::Wall => print!("#"),
    //             Cell::Space => print!("."),
    //             Cell::Portal => print!("p"),
    //         }
    //     }
    //     println!();
    // }

    (ans, portals)
}
