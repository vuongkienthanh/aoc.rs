use crate::parsing::parse_input;
use crate::step;

pub fn process(_input: &str) -> String {
    let (row, col) = run(_input);
    format!("{col},{row}")
}

fn run(input: &str) -> (usize, usize) {
    let (map, mut carts) = parse_input(input);
    loop {
        let crashes = step(&mut carts, &map);
        if !crashes.is_empty() {
            break crashes.into_iter().next().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let input = r#"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   "#;
        assert_eq!(run(input), (3, 7));
    }
}
