use crate::{ROCKS, run};

pub fn process(_input: &str) -> usize {
    let mut hole: Vec<u8> = vec![];
    let mut rocks = ROCKS.iter().cycle();
    let cmds: Vec<_> = _input.chars().collect();
    let mut cmdi = 0;
    let mut ans = 0;
    for _ in 0..2022 {
        let rock = rocks.next().cloned().unwrap();
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
        assert_eq!(process(fixture), 3068);
    }
}