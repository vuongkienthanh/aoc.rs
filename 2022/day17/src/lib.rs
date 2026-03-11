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