pub fn process(_input: &str) -> usize {
    let input = _input.replace("\n", "");
    input
        .split(',')
        .map(|group| {
            group
                .chars()
                .fold(0, |acc, c| (acc + c as usize) * 17 % 256)
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;
        assert_eq!(process(input), 1320);
    }
}
