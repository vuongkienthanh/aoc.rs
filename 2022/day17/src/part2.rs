use crate::{ROCKS, run, surface};
use fxhash::FxHashMap;

pub fn process(_input: &str) -> usize {
    let mut seen: FxHashMap<([Option<usize>; 7], usize, [u8; 4]), (usize, usize)> =
        FxHashMap::default();
    let mut hole: Vec<u8> = vec![];
    let mut rocks = ROCKS.iter().cycle();
    let cmds: Vec<_> = _input.chars().collect();
    let mut cmdi = 0;
    let mut ans = 0;
    for fall in 0..TARGET {
        let rock = rocks.next().cloned().unwrap();
        if let Some((old_fall, old_len)) =
            seen.insert((surface(&hole), cmdi, rock), (fall, ans + hole.len()))
        {
            let diff_fall = fall - old_fall;
            let diff_ans = ans + hole.len() - old_len;
            let div = (TARGET - fall) / diff_fall;
            let rem = (TARGET - fall) % diff_fall;
            ans += div * diff_ans;

            run(&mut hole, rock, &mut cmdi, &cmds, &mut ans);
            for _ in 1..rem {
                let rock = rocks.next().cloned().unwrap();
                run(&mut hole, rock, &mut cmdi, &cmds, &mut ans);
            }
            break;
        }
        run(&mut hole, rock, &mut cmdi, &cmds, &mut ans);
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
        assert_eq!(process(fixture), 1514285714288);
    }
}