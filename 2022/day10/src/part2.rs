use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let mut input = parse_input(_input).into_iter();
    let mut op = None;
    let mut sprite: [char; 40] = [
        '#', '#', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
        '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
        '.', '.', '.', '.',
    ];

    for _ in 0..6 {
        for i in 0..40 {
            print!("{}", sprite[i]);
            match op {
                None => op = input.next().unwrap(),
                Some(v) => {
                    if v >= 0 {
                        sprite.rotate_right(v.unsigned_abs());
                    } else {
                        sprite.rotate_left(v.unsigned_abs());
                    }
                    op = None;
                }
            }
        }

        println!();
    }

    0
}
