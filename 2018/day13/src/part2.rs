use crate::parsing::parse_input;
use crate::step;

pub fn process(_input: &str) -> String {
    let (row, col) = run(_input);
    format!("{col},{row}")
}

fn run(input: &str) -> (usize, usize) {
    let (map, mut carts) = parse_input(input);
    loop {
        step(&mut carts, &map);
        if carts.len() == 1 {
            break carts.into_keys().next().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_final_cart() {
        let input = r#"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"#;
        assert_eq!(run(input), (4, 6));
    }
}
