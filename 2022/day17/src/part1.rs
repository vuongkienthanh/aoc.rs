use crate::{ROCKS, do_cmd, is_conflicted};

pub fn process(_input: &str) -> usize {
    let mut hole: Vec<u8> = vec![];
    let mut rocks = ROCKS.iter().cycle();
    let cmds: Vec<_> = _input.chars().collect();
    let mut cmdi = 0;
    let mut ans = 0;
    for _ in 0..2022 {
        let len = hole.len();
        let mut rock = rocks.next().cloned().unwrap();
        //move rock 4 times and down 3 times
        for _ in 0..4 {
            rock = do_cmd(rock, &mut cmdi, &cmds);
        }
        // clash
        if len == 0 {
            hole.extend(rock);
        } else {
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
                let moved_rock = do_cmd(rock, &mut cmdi, &cmds);
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
        }

        while let Some(row) = hole.pop() {
            if row != 0 {
                hole.push(row);
                break;
            }
        }

        // case to_delete is first_row = 0-> can't find surface -> do nothing
        // case to_delete > 0 -> surface is to_delete -1 -> reallocate left right
        // case is last row (None) -> clear hole
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
                ans += to_delete;
                hole = hole[to_delete..].to_vec();
            }
        } else {
            ans += hole.len();
            hole.clear();
        }
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

