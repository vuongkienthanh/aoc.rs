use aoc_helper::adj::grid::adj4;
use grid::Grid;

pub fn process(_input: &str) -> u32 {
    let input = Grid::from(
        _input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|x| x.to_digit(10).unwrap() - 1)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );
    let rows = input.rows() * 5;
    let cols = input.cols() * 5;
    let start = (0, 0);
    let end = (rows - 1, cols - 1);
    let mut risk: Grid<u32> = Grid::new(rows, cols);
    risk.fill(u32::MAX);
    risk[start] = 0;

    let mut current = vec![start];
    while !current.is_empty() {
        let mut new = vec![];
        for p in current {
            for adj in adj4(p, rows, cols).into_iter().flatten() {
                let risk_at_adj = (input[(adj.0 % input.rows(), adj.1 % input.cols())]
                    + (adj.0 / input.rows()) as u32
                    + (adj.1 / input.cols()) as u32)
                    % 9;
                let new_risk = risk[p] + risk_at_adj + 1;
                if new_risk < risk[adj] {
                    risk[adj] = new_risk;
                    new.push(adj);
                }
            }
        }
        current = new;
    }

    risk[end]
}
