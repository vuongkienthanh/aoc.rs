use aoc_helper::adj::grid::adj4;
use grid::Grid;

pub fn process(_input: &str) -> u32 {
    let input = Grid::from(
        _input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|x| x.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );
    let rows = input.rows();
    let cols = input.cols();
    let start = (0, 0);
    let end = (input.rows() - 1, input.cols() - 1);
    let mut risk: Grid<u32> = Grid::new(rows, cols);
    risk.fill(u32::MAX);
    risk[start] = 0;

    let mut current = vec![start];
    while !current.is_empty() {
        let mut new = vec![];
        for p in current {
            for adj in adj4(p, input.rows(), input.cols()).into_iter().flatten() {
                let new_risk = risk[p] + input[adj];
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
