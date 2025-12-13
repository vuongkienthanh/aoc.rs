use super::parse;
use grid::Grid;
pub fn process(_input: &str, rows: usize, cols: usize) {
    let mut grid = Grid::<usize>::new(rows, cols);
    let (rows, cols) = (rows as isize, cols as isize);
    let (_, input) = parse(_input.trim()).unwrap();
    // check visually
    // first vertical and horizontal
    // should appear under 101 or 103 first iterations
    for i in 0..100 {
        // first vertical is 97
        let j = 97 + i * cols;
        // first horizontal is 50
        if j % rows == 50 {
            for (p, v) in input.iter() {
                let mut new_p = (p.0 + j * v.0, p.1 + j * v.1);
                new_p = (
                    ((new_p.0 % rows) + rows) % rows,
                    ((new_p.1 % cols) + cols) % cols,
                );
                grid[(new_p.0 as usize, new_p.1 as usize)] += 1;
            }
            println!("======================================================================");
            println!("j={j}");
            check_chrismas_tree(&grid);
            grid.fill(0);
        }
    }
}

fn check_chrismas_tree(grid: &Grid<usize>) {
    for row in grid.iter_rows() {
        for cell in row.into_iter() {
            if cell == &0 {
                print!(" ");
            } else {
                print!("*");
            }
        }
        println!();
    }
}
