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
fn do_cmd(rock: [u8; 4], cmd: char) -> [u8; 4] {
    match cmd {
        '>' => move_right(rock),
        '<' => move_left(rock),
        _ => panic!(),
    }
}

pub fn process(_input: &str) -> usize {
    let mut hole: Vec<u8> = vec![];
    let mut rocks = ROCKS.iter().cycle();
    let mut cmds = _input.chars().cycle();
    let mut ans = 0;
    for i in 0..10 {
        println!("{i}");
        let len = hole.len();
        let mut rock = rocks.next().cloned().unwrap();
        //move rock 4 times and down 3 times
        for _ in 0..4 {
            rock = do_cmd(rock, cmds.next().unwrap());
        }
        for row in rock.iter().rev() {
            println!("{row:07b}");
        }
        // clash
        if len == 0 {
            hole.extend(rock);
        } else {
            for i in (0..len).rev() {
                //down
                if i==0 || !hole[i..len].iter().zip(rock).all(|(a, b)| {
                    let fuse = a | b;
                    fuse.count_ones() == a.count_ones() + b.count_ones()
                }) {

                    for _ in 0..5usize.saturating_sub(len - i) {
                        hole.push(0);
                    }
                    hole[i + 1..]
                        .iter_mut()
                        .zip(rock)
                        .for_each(|(a, b)| *a |= b);
                    break;
                }

                // then move
                let moved_rock = do_cmd(rock, cmds.next().unwrap());

                rock = if hole[i..len].iter().zip(moved_rock).all(|(a, b)| {
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
        for (i, row) in hole.iter().enumerate().rev() {
            println!("{i} {row:07b}");
        }
        println!();
        let to_delete = hole
            .iter()
            .enumerate()
            .rev()
            .scan((0, 0), |(_, state), (i, row)| {
                *state |= row;
                if *state == 0b1111111 {
                    return None;
                }
                Some((i, *state))
            })
            .map(|(i, _)| i)
            .last()
            .unwrap();

        let hole2 = hole.split_off(to_delete);
        ans += to_delete;
        hole = hole2;
    }

    ans + hole.len()
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

