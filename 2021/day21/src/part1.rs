use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (p1, p2) = parse_input(_input);
    let (mut p1, mut p2) = (p1 - 1, p2 - 1);
    let (mut p1s, mut p2s) = (0, 0);
    let mut die = (1..=100).cycle();
    let mut rolled = 0;
    let mut p1_turn = true;

    loop {
        let (p, s) = if p1_turn {
            (&mut p1, &mut p1s)
        } else {
            (&mut p2, &mut p2s)
        };
        let r0 = die.next().unwrap();
        let r1 = die.next().unwrap();
        let r2 = die.next().unwrap();
        rolled += 3;
        *p += r0 + r1 + r2;
        *p %= 10;
        *s += *p + 1;
        if *s >= 1000 {
            break;
        }
        p1_turn = !p1_turn;
    }
    if p1_turn { p2s * rolled } else { p1s * rolled }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"Player 1 starting position: 4
Player 2 starting position: 8"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 739785);
    }
}
