use crate::PipeMaze;
pub fn process(_input: &str) -> usize {
     PipeMaze::new(_input).count() / 2
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_1() {
        let input = r#"-L|F
7S-7|
L|7||
-L-J|
L|-JF"#;
        assert_eq!(process(input), 4);
    }
    #[test]
    fn test_process_2() {
        let input = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;
        assert_eq!(process(input), 8);
    }
}
