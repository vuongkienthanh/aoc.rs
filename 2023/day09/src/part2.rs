use crate::parse_input;

pub fn process(_input: &str) -> isize {
    parse_input(_input)
        .into_iter()
        .map(|line| contra_extrapolate(line))
        .sum()
}

pub fn contra_extrapolate(input: Vec<isize>) -> isize {
    if input.iter().all(|n| n == &0) {
        return 0;
    }
    let next_seq = (0..input.len() - 1)
        .map(|i| input.get(i + 1).unwrap() - input.get(i).unwrap())
        .collect::<Vec<isize>>();
    input.get(0).unwrap() - contra_extrapolate(next_seq)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        assert_eq!(process(input), 2);
    }
}
