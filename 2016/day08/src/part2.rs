use crate::parsing::parse_input;
use crate::{COLS, ROWS, swipe};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    let mut screen = [[0; COLS]; ROWS];

    swipe(&mut screen, input);

    for row in screen {
        for (i, ele) in row.iter().enumerate() {
            if *ele == 1 {
                print!("*");
            } else {
                print!(" ");
            }
            if (i + 1) % 5 == 0 {
                print!("   ");
            }
        }
        println!();
    }

    0
}
