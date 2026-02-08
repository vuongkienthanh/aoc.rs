use std::collections::BTreeMap;

type Point = (usize, usize);
type Portal = (char, char);

#[derive(Debug, Clone)]
pub enum Cell {
    Space,
    Wall,
    InnerPortal(Portal),
    OuterPortal(Portal),
}

pub fn parse(input: &str) -> (Vec<Vec<Cell>>, BTreeMap<Portal, Vec<Point>>, Point, Point) {
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let rows = input.len() - 4;
    let cols = input[0].len() - 4;
    let mut ans = vec![vec![Cell::Wall; cols]; rows];
    let mut portals: BTreeMap<Portal, Vec<Point>> = BTreeMap::new();
    let mut start = None;
    let mut end = None;

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
                let (cc, (r, c)) = if u.is_alphabetic() && d == '.' {
                    ((u, m), (row - 1, col - 2))
                } else if u == '.' && d.is_alphabetic() {
                    ((m, d), (row - 3, col - 2))
                } else if l.is_alphabetic() && r == '.' {
                    ((l, m), (row - 2, col - 1))
                } else if l == '.' && r.is_alphabetic() {
                    ((m, r), (row - 2, col - 3))
                } else {
                    continue;
                };
                match cc {
                    ('A', 'A') => start = Some((r, c)),
                    ('Z', 'Z') => end = Some((r, c)),
                    _ => {
                        if r == 0 || c == 0 || r == ans.len() - 1 || c == ans[0].len() - 1 {
                            ans[r][c] = Cell::OuterPortal(cc);
                        } else {
                            ans[r][c] = Cell::InnerPortal(cc);
                        }
                        portals.entry(cc).or_default().push((r, c));
                    }
                }
            } else if m == '.' && matches!(ans[row - 2][col - 2], Cell::Wall) {
                ans[row - 2][col - 2] = Cell::Space;
            }
        }
    }

    // for row in &ans {
    //     for cell in row {
    //         match cell {
    //             Cell::Wall => print!("#"),
    //             Cell::Space => print!("."),
    //             Cell::OuterPortal(_) => print!("o"),
    //             Cell::InnerPortal(_) => print!("x"),
    //         }
    //     }
    //     println!();
    // }

    (ans, portals, start.unwrap(), end.unwrap())
}
