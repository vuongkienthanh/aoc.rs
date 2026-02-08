pub mod parsing;
pub mod part1;
pub mod part2;

use parsing::Action;
const ROWS: usize = 6;
const COLS: usize = 50;

fn swipe(screen: &mut [[usize; COLS]; ROWS], input: Vec<Action>) {
    for action in input {
        match action {
            Action::Rect(wide, tall) =>
            {
                #[allow(clippy::needless_range_loop)]
                for i in 0..tall {
                    for j in 0..wide {
                        screen[i][j] = 1;
                    }
                }
            }
            Action::RotateRow(row, by) => {
                let mut new_pixels = [0; COLS];
                for col in 0..COLS {
                    if screen[row][col] == 1 {
                        new_pixels[(col + by) % COLS] = 1;
                    }
                }
                screen[row].copy_from_slice(&new_pixels);
            }
            Action::RotateCol(col, by) => {
                let mut new_pixels = [0; ROWS];
                for row in 0..ROWS {
                    if screen[row][col] == 1 {
                        new_pixels[(row + by) % ROWS] = 1;
                    }
                }
                for row in 0..ROWS {
                    screen[row][col] = new_pixels[row];
                }
            }
        }
    }
}
