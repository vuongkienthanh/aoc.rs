const ROCKS: [[u8; 4]; 5] = [
    [0b11110, 0, 0, 0],
    [0b01000, 0b11100, 0b01000, 0],
    [0b11100, 0b00100, 0b00100, 0],
    [0b10000, 0b10000, 0b10000, 0b10000],
    [0b11000, 0b11000, 0, 0],
];

fn move_left(rock: [u8; 4]) -> [u8; 4] {
    if rock.iter().all(|x| x & (1 << 6) == 0) {
        [rock[0] << 1, rock[1] << 1, rock[2] << 1, rock[3] << 1]
    } else {
        rock
    }
}
fn move_right(rock: [u8; 4]) -> [u8; 4] {
    if rock.iter().all(|x| x & 1 == 0) {
        [rock[0] >> 1, rock[1] >> 1, rock[2] >> 1, rock[3] >> 1]
    } else {
        rock
    }
}

pub fn process(_input: &str) -> usize {
    let mut hole: Vec<u8> = vec![];
    let mut rocks = ROCKS.iter().cycle();
    let mut cmds = _input.chars().cycle();
    for i in 0..2 {
        println!("{i}");
        let len = hole.len();
        let mut rock = rocks.next().cloned().unwrap();
        //move rock 3 times and down 3 times
        for _ in 0..3 {
            match cmds.next().unwrap() {
                '>' => rock = move_right(rock),
                '<' => rock = move_left(rock),
                _ => panic!(),
            }
        }
        // clash
        if len == 0 {
            match cmds.next().unwrap() {
                '>' => rock = move_right(rock),
                '<' => rock = move_left(rock),
                _ => panic!(),
            }
            hole.extend(rock);
        } else {
            for i in (len.saturating_sub(4)..len).rev() {
                let clash_area = &hole[i..len];
                let moved_rock = match cmds.next().unwrap() {
                    '>' => move_right(rock),
                    '<' => move_left(rock),
                    _ => panic!(),
                };

                let rock_after_cmd = if clash_area.iter().zip(moved_rock).all(|(a, b)| {
                    let fuse = a | b;
                    fuse.count_ones() == a.count_ones() + b.count_ones()
                }) {
                    moved_rock
                } else {
                    rock
                };
            }
        }

        while let Some(row) = hole.pop() {
            if row != 0 {
                hole.push(row);
                break;
            }
        }

        println!("hole = ");
        for row in &hole {
            println!("{row:07b}");
        }
        println!();
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#
    }

    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 3068);
    }
}
