pub mod part1;
pub mod part2;

pub const ROCKS: [[u8; 4]; 5] = [
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

fn do_cmd(rock: [u8; 4], cmdi: &mut usize, cmds: &[char]) -> [u8; 4] {
    let cmd = cmds[*cmdi];
    *cmdi += 1;
    *cmdi %= cmds.len();
    match cmd {
        '>' => move_right(rock),
        '<' => move_left(rock),
        _ => panic!(),
    }
}

pub fn print_obj(obj: &[u8]) {
    for (i, row) in obj.iter().enumerate().rev() {
        println!("{i} {row:07b}");
    }
}

pub fn is_conflicted(hole: &[u8], base_row: usize, rock: &[u8]) -> bool {
    !hole[base_row..].iter().zip(rock).all(|(a, b)| {
        let fuse = a | b;
        fuse.count_ones() == a.count_ones() + b.count_ones()
    })
}

pub fn surface(hole: &[u8]) -> [Option<usize>; 7] {
    let mut ans: [Option<usize>; 7] = [None; 7];
    for (i, row) in hole.iter().enumerate().rev() {
        for col in 0..7 {
            if row & (1 << (6 - col)) == (1 << (6 - col)) {
                if ans[col].is_none() {
                    ans[col] = Some(i);
                }
            }
        }
        if ans.iter().all(|x| x.is_some()) {
            break;
        }
    }
    ans
}

pub fn run(hole: &mut Vec<u8>, mut rock: [u8; 4], cmdi: &mut usize, cmds: &[char], ans: &mut usize) {
    let len = hole.len();
    //move rock 4 times and down 3 times
    for _ in 0..4 {
        rock = do_cmd(rock, cmdi, cmds);
    }
    // clash
    let mut rest = false;
    for i in (0..len).rev() {
        //down
        if is_conflicted(&hole, i, &rock) {
            for _ in 0..5usize.saturating_sub(len - i) {
                hole.push(0);
            }
            hole[i + 1..]
                .iter_mut()
                .zip(rock)
                .for_each(|(a, b)| *a |= b);
            rest = true;
            break;
        }
        // then move left or right
        let moved_rock = do_cmd(rock, cmdi, cmds);
        rock = if is_conflicted(&hole, i, &moved_rock) {
            rock
        } else {
            moved_rock
        };
    }
    // if all the way down with no rest
    if !rest {
        for _ in 0..4usize.saturating_sub(len) {
            hole.push(0);
        }

        hole.iter_mut().zip(rock).for_each(|(a, b)| *a |= b);
    }
    // rm padding
    while let Some(row) = hole.pop() {
        if row != 0 {
            hole.push(row);
            break;
        }
    }

    // keep hole small
    if let Some(to_delete) = hole
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
    {
        if to_delete > 0 {
            *ans += to_delete;
            *hole = hole[to_delete..].to_vec();
        }
    } else {
        *ans += hole.len();
        hole.clear();
    }
}