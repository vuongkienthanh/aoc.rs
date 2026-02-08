pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    for d in 1.. {
        if passthrough_with_delay(&input, d) {
            return d;
        }
    }
    panic!("should have an answer")
}
fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .collect()
}

fn passthrough_with_delay(input: &[(usize, usize)], d: usize) -> bool {
    input.into_iter().all(|(a, b)| {
        let range = (b - 1) * 2;
        (a + d) % range != 0
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"0: 3
1: 2
4: 4
6: 4"#
    }
    #[rstest]
    fn test_delay(fixture: &str) {
        let input = parse(fixture);
        assert_eq!(passthrough_with_delay(&input, 10), true);
    }
    #[rstest]
    fn test_process(fixture: &str) {
        assert_eq!(process(fixture), 10);
    }
}
