use grid::Grid;
use aoc_helper::adj::{
    grid::{adj_corners, adj_edges, adj_inner},
    naive::adj4,
};

pub fn process(_input: &str) -> usize {
    let input = Grid::from(_input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>());
    let rows = input.rows();
    let cols = input.cols();
    let mut not_lowest = Grid::from(vec![vec![false; cols]; rows]);
    let mut ans = 0;
    for (p, [adj1, adj2]) in adj_corners(rows, cols) {
        if input[p] < input[adj1] && input[p] < input[adj2]
        {
            ans += input[p] as usize + 1;
            not_lowest[adj1] = true;
            not_lowest[adj2] = true;
        }
    }

    for (p, [adj1, adj2, adj3]) in adj_edges(rows, cols) {
        if not_lowest[p] {
            continue;
        }
        if input[p] < input[adj1] && input[p] < input[adj2] && input[p] < input[adj3] {
            ans += input[p] as usize + 1;
            not_lowest[adj1] = true;
            not_lowest[adj2] = true;
            not_lowest[adj3] = true;
        }
    }

    for (p, [adj1, adj2, adj3, adj4]) in adj_inner(rows, cols) {
        if not_lowest[p] {
            continue;
        }
        if input[p] < input[adj1]
            && input[p] < input[adj2]
            && input[p] < input[adj3]
            && input[p] < input[adj4]
        {
            ans += input[p] as usize + 1;
            not_lowest[adj1] = true;
            not_lowest[adj2] = true;
            not_lowest[adj3] = true;
            not_lowest[adj4] = true;
        }
    }
    ans
}