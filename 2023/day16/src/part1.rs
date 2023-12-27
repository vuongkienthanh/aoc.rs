use crate::{Beam, Direction, Puzzle};

pub fn process(_input: &str) -> usize {
    let beam = Beam {
        loc: (0, 0),
        direction: Direction::Right,
    };

    Puzzle::new(_input).run(beam)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!(process(input), 46);
    }
}
