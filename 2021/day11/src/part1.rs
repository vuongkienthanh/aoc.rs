use aoc_helper::adj::grid::adj8;
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let mut grid = Grid::from(
        _input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|x| x.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );
    let mut ans = 0;

    for _ in 0..100 {
        grid.iter_mut().for_each(|x| *x += 1);
        let mut flashed = true;
        while flashed {
            flashed = false;
            for row in 0..10 {
                for col in 0..10 {
                    if grid[(row, col)] > 9 {
                        flashed = true;
                        ans += 1;
                        grid[(row, col)] = 0;
                        for (r, c) in adj8((row, col), 10, 10).into_iter().flatten() {
                            if grid[(r, c)] != 0 {
                                grid[(r, c)] += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    ans
}
