/// rayon doesn't speed up this puzzle solving time.
///
/// time std iter
/// user    0m0.690s
///
/// time rayon par_iter
/// user    0m8.867s
///

// use rayon::prelude::*; 
pub fn process(_input: &str) -> usize {
    let mut input = _input.lines().map(|line| {
        line.split_ascii_whitespace()
            .skip(1)
            .collect::<Vec<_>>()
            .join("")
            .parse::<usize>()
            .unwrap()
    });
    let time = input.next().unwrap();
    let distance = input.next().unwrap();

    (0..=time)
        // .into_par_iter()
        .map(|hold_time| {
            hold_time * (time - hold_time)
        })
        .filter(|travel| travel > &distance)
        .count()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        assert_eq!(process(input), 71503);
    }
}
