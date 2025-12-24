use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let mut input = parse_input(_input);
    let mut step = 0;
    let mut i = 0usize;
    'a: loop {
        step += 1;
        let cmd = input[i];
        input[i] += 1;
        let new_i = i.checked_add_signed(cmd);
        match new_i {
            None => break 'a step,
            Some(x) if x >= input.len() => break 'a step,
            Some(x) => i = x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"0
3
0
1
-3"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 5);
    }
}
