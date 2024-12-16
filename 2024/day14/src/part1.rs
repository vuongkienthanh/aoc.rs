use super::parse;
pub fn process(_input: &str, rows: usize, cols: usize) -> usize {
    let (rows, cols) = (rows as isize, cols as isize);
    let (_, input) = parse(_input.trim()).unwrap();

    input
        .into_iter()
        .map(|(p, v)| {
            let new_p = (p.0 + 100 * v.0, p.1 + 100 * v.1);
            (
                ((new_p.0 % rows) + rows) % rows,
                ((new_p.1 % cols) + cols) % cols,
            )
        })
        .fold([0, 0, 0, 0], |mut acc, p| {
            match (p.0.cmp(&(rows / 2)), p.1.cmp(&(cols / 2))) {
                (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => acc[0] += 1,
                (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => acc[1] += 1,
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => acc[2] += 1,
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => acc[3] += 1,
                _ => (),
            }
            acc
        })
        .into_iter()
        .product()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;
        assert_eq!(process(input, 7, 11), 12);
    }
}
