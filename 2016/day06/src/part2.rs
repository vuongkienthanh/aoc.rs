pub fn process(_input: &str) -> String {
    _input
        .lines()
        .fold(Vec::<[usize; 26]>::new(), |mut acc, line| {
            for (i, c) in line.bytes().enumerate() {
                if let Some(column) = acc.get_mut(i) {
                    *column.get_mut((c - b'a') as usize).unwrap() += 1;
                } else {
                    let mut column = [0; 26];
                    *column.get_mut((c - b'a') as usize).unwrap() += 1;
                    acc.push(column)
                }
            }
            acc
        })
        .into_iter()
        .map(|column| {
            ('a'..='z')
                .zip(column)
                .min_by_key(|(_, x)| *x)
                .map(|(c, _)| c)
                .unwrap()
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar"#
    }
    #[rstest]
    fn test_process_1(fixture: &str) {
        assert_eq!(process(fixture), "advent");
    }
}
