pub fn process(_input: &str) -> usize {
    let mut lines = _input.lines();
    let times = lines.next().unwrap().split_ascii_whitespace().skip(1);
    let distances = lines.next().unwrap().split_ascii_whitespace().skip(1);

    times
        .zip(distances)
        .map(|(time, distance)| {
            (
                time.parse::<usize>().unwrap(),
                distance.parse::<usize>().unwrap(),
            )
        })
        .map(|(time, distance)| {
            (0..=time)
                .map(|hold_time| {
                    // get travel
                    hold_time * (time - hold_time)
                })
                .filter(|travel| travel > &distance)
                .count()
        })
        .product()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        assert_eq!(process(input), 288);
    }
}
