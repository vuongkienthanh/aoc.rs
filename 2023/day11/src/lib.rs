pub mod part1;
pub mod part2;

use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug)]
struct GalaxyMap {
    galaxies: VecDeque<(usize, usize)>,
    blank_col_indices: Vec<usize>,
    blank_row_indices: Vec<usize>,
}

impl GalaxyMap {
    fn new(input: &str) -> Self {
        let row_count = input.lines().count();
        let col_count = input.lines().next().unwrap().len();

        let blank_row_indices = (0..row_count)
            .filter_map(|i| {
                if input.lines().nth(i).unwrap().chars().all(|c| c == '.') {
                    Some(i)
                } else {
                    None
                }
            })
            .rev()
            .collect::<Vec<_>>();

        let blank_col_indices = (0..col_count)
            .filter_map(|j| {
                if input
                    .lines()
                    .map(|line| line.chars().nth(j).unwrap())
                    .all(|c| c == '.')
                {
                    Some(j)
                } else {
                    None
                }
            })
            .rev()
            .collect::<Vec<_>>();

        let galaxies = (0..row_count)
            .flat_map(|i| (0..col_count).map(move |j| (i, j)))
            .filter_map(|(i, j)| {
                if input.lines().nth(i).unwrap().chars().nth(j).unwrap() == '#' {
                    Some((i, j))
                } else {
                    None
                }
            })
            .collect::<VecDeque<_>>();

        Self {
            galaxies,
            blank_col_indices,
            blank_row_indices,
        }
    }

    fn expand(&mut self, level: usize) {
        for idx in &self.blank_col_indices {
            for _ in 0..self.galaxies.len() {
                let galaxy = self.galaxies.pop_front().unwrap();
                let new_galaxy = if galaxy.1 > *idx {
                    (galaxy.0, galaxy.1 + level - 1)
                } else {
                    galaxy
                };
                self.galaxies.push_back(new_galaxy);
            }
        }
        for idx in &self.blank_row_indices {
            for _ in 0..self.galaxies.len() {
                let galaxy = self.galaxies.pop_front().unwrap();
                let new_galaxy = if galaxy.0 > *idx {
                    (galaxy.0 + level - 1, galaxy.1)
                } else {
                    galaxy
                };
                self.galaxies.push_back(new_galaxy);
            }
        }
    }

    fn solve(&mut self, level: usize) -> usize {
        self.expand(level);
        self.galaxies
            .iter()
            .combinations(2)
            .map(|pair| {
                let p0 = pair.get(0).unwrap();
                let p1 = pair.get(1).unwrap();

                p1.0.abs_diff(p0.0) + p1.1.abs_diff(p0.1)
            })
            .sum()
    }
}
