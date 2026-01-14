pub mod part1;
pub mod part2;

use aoc_helper::adj::grid::adj8;

pub type Point = (usize, usize);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Acre {
    Open,
    Wood,
    LumberYard,
}

impl From<char> for Acre {
    fn from(value: char) -> Self {
        match value {
            '.' => Acre::Open,
            '|' => Acre::Wood,
            '#' => Acre::LumberYard,
            _ => panic!(),
        }
    }
}

impl Acre {
    pub fn transform(&self, loc: Point, map: &Vec<Vec<Acre>>) -> Self {
        let rows = map.len();
        let cols = map[0].len();
        match self {
            Acre::Open => {
                if adj8(loc, rows, cols)
                    .into_iter()
                    .flatten()
                    .filter(|(r, c)| matches!(map[*r][*c], Acre::Wood))
                    .count()
                    >= 3
                {
                    Acre::Wood
                } else {
                    Acre::Open
                }
            }
            Acre::Wood => {
                if adj8(loc, rows, cols)
                    .into_iter()
                    .flatten()
                    .filter(|(r, c)| matches!(map[*r][*c], Acre::LumberYard))
                    .count()
                    >= 3
                {
                    Acre::LumberYard
                } else {
                    Acre::Wood
                }
            }
            Acre::LumberYard => {
                let mut lumberyard = 0;
                let mut wood = 0;
                for (r, c) in adj8(loc, rows, cols).into_iter().flatten() {
                    if matches!(map[r][c], Acre::LumberYard) {
                        lumberyard += 1;
                    }
                    if matches!(map[r][c], Acre::Wood) {
                        wood += 1;
                    }
                }
                if lumberyard >= 1 && wood >= 1 {
                    Acre::LumberYard
                } else {
                    Acre::Open
                }
            }
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Acre>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect()
}

fn next(outskirts: Vec<Vec<Acre>>) -> Vec<Vec<Acre>> {
    let mut new_outskirts = vec![];

    for (r, line) in outskirts.iter().enumerate() {
        let mut new_row = vec![];

        for (c, acre) in line.iter().enumerate() {
            new_row.push(acre.transform((r, c), &outskirts));
        }

        new_outskirts.push(new_row);
    }
    new_outskirts
}

fn resource_value(outskirts: Vec<Vec<Acre>>) -> usize {
    let mut wood = 0;
    let mut lumberyard = 0;
    for row in outskirts {
        for acre in row {
            if matches!(acre, Acre::Wood) {
                wood += 1;
            }
            if matches!(acre, Acre::LumberYard) {
                lumberyard += 1;
            }
        }
    }
    wood * lumberyard
}
