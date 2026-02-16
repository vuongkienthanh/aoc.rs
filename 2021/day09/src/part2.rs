use aoc_helper::adj::grid::adj4;
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let input = Grid::from(
        _input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|x| x.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );
    let rows = input.rows();
    let cols = input.cols();
    let mut seen = Grid::from(vec![vec![false; cols]; rows]);
    let mut basins = vec![];
    for (p, cell) in input.indexed_iter() {
        if seen[p] {
            continue;
        }
        let adjs: Vec<(usize, usize)> = adj4(p, rows, cols).into_iter().flatten().collect();
        if adjs.iter().all(|p| input[*p] > *cell) {
            seen[p] = true;
            basins.push(bfs(adjs, &input, &mut seen));
        }
    }
    basins.sort_unstable();
    let len = basins.len();
    basins.into_iter().skip(len - 3).product()
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
                for new_p in adj4(p, grid.rows(), grid.cols()).into_iter().flatten() {
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

