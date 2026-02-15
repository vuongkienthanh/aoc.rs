use grid::Grid;
use aoc_helper::adj::grid::{adj_corners, adj_edges, adj_inner, adj4};

pub fn process(_input: &str) -> usize {
    let input = _input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let rows = input.len();
    let cols = input.first().unwrap().len();
    let mut seen = Grid::from(vec![vec![false; cols]; rows]);
    let mut basins = vec![];
    for (p, [adj1, adj2]) in adj_corners(rows, cols) {
        seen[p] = true;
        if input[p] < input[adj1] && input[p] < input[adj2] {
            basins.push(bfs(vec![adj1, adj2], &input, &mut seen));
        }
    }
    for (p, [adj1, adj2, adj3]) in adj_edges(rows, cols) {
        if seen[p] {
            continue;
        }
        seen[p] = true;
        if input[p] < input[adj1] && input[p] < input[adj2] && input[p] < input[adj3] {
            basins.push(bfs(vec![adj1, adj2, adj3], &input, &mut seen));
        }
    }
    for (p, [adj1, adj2, adj3, adj4]) in adj_inner(rows, cols) {
        if seen[p] {
            continue;
        }
        seen[p] = true;
        if input[p] < input[adj1]
            && input[p] < input[adj2]
            && input[p] < input[adj3]
            && input[p] < input[adj4]
        {
            basins.push(bfs(vec![adj1, adj2, adj3, adj4], &input, &mut seen));
        }
    }
    basins.sort_unstable();
    basins.into_iter().skip(input.len() - 3).sum()
}

fn bfs(mut current: Vec<(usize, usize)>, grid: &Grid<u8>, seen: &mut Grid<bool>) -> usize {
    let mut ans = 1;
    while !current.is_empty() {
        let mut new = vec![];
        for p in current {
            if grid[p] == 9 {
                continue;
            }
            if !seen[p] {
                seen[p] = true;
                ans += 1;
                for new_p in adj4(p, grid.rows(), grid.cols()) {
                    if grid[new_p] >= grid[p] {
                        new.push(new_p);
                    }
                }
            }
        }
        current = new;
    }
    ans
}